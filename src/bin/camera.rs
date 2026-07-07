//! Live camera viewfinder for the WeAct MiniSTM32H7xx.
//!
//! Data flow (classic ping-pong, two frame buffers in AXI SRAM):
//!
//! ```text
//! OV2640 --DVP--> DCMI+DMA --[FILLED]--> display task (main) --> ST7735
//!                    ^                        |
//!                    +--------[FREE]----------+
//! ```
//!
//! While one buffer is being shown, the DCMI captures into the other, so
//! the pipeline never copies a frame. A heartbeat LED task proves the
//! executor is alive independently of the camera path.

#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::dcmi::{self, Dcmi};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::mode::Async;
use embassy_stm32::rcc::{Mco, Mco1Source, McoConfig};
use embassy_stm32::spi::{self, Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, dma, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Instant, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use static_cell::ConstStaticCell;

use weact_h743::display::{self, FRAME_H, FRAME_W, FRAME_WORDS};
use weact_h743::ov2640::{self, Ov2640};
use weact_h743::board;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DCMI => dcmi::InterruptHandler<peripherals::DCMI>;
    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
    DMA1_STREAM1 => dma::InterruptHandler<peripherals::DMA1_CH1>;
    DMA1_STREAM2 => dma::InterruptHandler<peripherals::DMA1_CH2>;
});

type FrameBuf = [u32; FRAME_WORDS];
type CamI2c = I2c<'static, Async, i2c::mode::Master>;

// .bss, AXI SRAM — reachable by the DCMI's DMA, unlike DTCM.
static BUF_A: ConstStaticCell<FrameBuf> = ConstStaticCell::new([0; FRAME_WORDS]);
static BUF_B: ConstStaticCell<FrameBuf> = ConstStaticCell::new([0; FRAME_WORDS]);

static FREE: Channel<CriticalSectionRawMutex, &'static mut FrameBuf, 2> = Channel::new();
static FILLED: Channel<CriticalSectionRawMutex, &'static mut FrameBuf, 2> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(board::config());
    info!("WeAct H743 camera: {}x{} RGB565 viewfinder", FRAME_W, FRAME_H);

    spawner.spawn(unwrap!(heartbeat(Output::new(p.PE3, Level::Low, Speed::Low))));

    // --- Viewfinder first: it doubles as the error console. ---
    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = Hertz(24_000_000);
    let spi_bus = Spi::new_blocking_txonly(p.SPI4, p.PE12, p.PE14, spi_cfg);
    let cs = Output::new(p.PE11, Level::High, Speed::VeryHigh);
    let spi_dev = unwrap!(ExclusiveDevice::new_no_delay(spi_bus, cs).map_err(|_| ()));
    let dc = Output::new(p.PE13, Level::Low, Speed::VeryHigh);
    // The panel resets with NRST; the driver still wants a reset pin, so it
    // gets an unused GPIO (same trick as the clock firmware).
    let rst = Output::new(p.PE15, Level::High, Speed::Low);

    let mut lcd = unwrap!(display::init_lcd(spi_dev, dc, rst));
    display::show_error(&mut lcd, "CAM INIT...");
    // Backlight on (active low through a P-MOSFET). Held for the whole run.
    let _backlight = Output::new(p.PE10, Level::Low, Speed::Low);

    // --- Camera: XCLK must run before the sensor will answer on SCCB. ---
    let mco_cfg = {
        let mut c = McoConfig::default();
        c.prescaler = board::XCLK_PRESCALER;
        c
    };
    let _xclk = Mco::new(p.MCO1, p.PA8, Mco1Source::HSE, mco_cfg);
    Timer::after_millis(10).await;

    let cam_i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        p.DMA1_CH1,
        p.DMA1_CH2,
        Irqs,
        i2c::Config::default(), // 100 kHz, well within SCCB spec
    );

    // Keep the sensor handle alive: dropping it would release the SCCB pins.
    let _cam = match camera_setup(cam_i2c).await {
        Ok(cam) => cam,
        Err(e) => {
            warn!("camera setup failed: {}", e);
            display::show_error(&mut lcd, "NO CAMERA");
            // Heartbeat keeps blinking so "no camera" != "firmware dead".
            loop {
                Timer::after_secs(1).await;
            }
        }
    };

    let dcmi = Dcmi::new_8bit(
        p.DCMI,
        p.DMA1_CH0,
        Irqs,
        p.PC6, // D0
        p.PC7, // D1
        p.PE0, // D2
        p.PE1, // D3
        p.PE4, // D4
        p.PD3, // D5
        p.PE5, // D6
        p.PE6, // D7
        p.PB7, // VSYNC
        p.PA4, // HSYNC
        p.PA6, // PIXCLK
        dcmi::Config::default(), // VSYNC/HREF/PCLK polarity: OV2640 defaults
    );

    unwrap!(FREE.try_send(BUF_A.take()).map_err(|_| ()));
    unwrap!(FREE.try_send(BUF_B.take()).map_err(|_| ()));
    spawner.spawn(unwrap!(capture(dcmi)));

    // --- Main task becomes the display pump. ---
    let mut frames: u32 = 0;
    let mut t0 = Instant::now();
    loop {
        let buf = FILLED.receive().await;
        display::draw_frame(&mut lcd, buf);
        FREE.send(buf).await;

        frames += 1;
        if frames == 32 {
            let ms = t0.elapsed().as_millis() as u32;
            info!("{} fps x10", (frames * 10_000) / ms.max(1));
            frames = 0;
            t0 = Instant::now();
        }
    }
}

/// Probe + full configuration of the sensor: RGB565, 320x160 (2:1 to
/// match the panel; the DSP crops/downscales from the SVGA frame).
async fn camera_setup(
    i2c: CamI2c,
) -> Result<Ov2640<CamI2c>, ov2640::Error<i2c::Error>> {
    let mut cam = Ov2640::probe(i2c).await?;
    cam.init().await?;
    cam.set_pixel_format_rgb565().await?;
    cam.set_output_size(FRAME_W as u16, FRAME_H as u16).await?;
    // For bring-up without optics/light, swap in the test pattern:
    // cam.set_colorbar(true).await?;
    Ok(cam)
}

#[embassy_executor::task]
async fn capture(mut dcmi: Dcmi<'static, peripherals::DCMI>) {
    loop {
        let buf = FREE.receive().await;
        match dcmi.capture(&mut buf[..]).await {
            Ok(()) => FILLED.send(buf).await,
            Err(e) => {
                warn!("DCMI capture error: {}", e);
                FREE.send(buf).await;
                Timer::after_millis(100).await;
            }
        }
    }
}

/// 1 Hz blink = executor alive. Fails independently of the camera path.
#[embassy_executor::task]
async fn heartbeat(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
