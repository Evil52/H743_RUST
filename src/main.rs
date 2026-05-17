#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32h7xx_hal::{
    delay::Delay,
    gpio::{gpioe::PE3, Output, PushPull},
    pac::{self, interrupt, TIM2},
    prelude::*,
    rtc::{self, RtcClock},
    spi,
    timer::{Event, Timer},
};

use chrono::{Datelike, NaiveDate, Timelike};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use st7735_lcd::{Orientation, ST7735};

// WeAct STM32H743VIT6 Mini — onboard 0.96" 80x160 ST7735:
//   SPI4: SCK = PE12, MOSI = PE14
//   LCD_CS    = PE11
//   LCD_DC    = PE13
//   LCD_BL    = PE10 (active-low via P-MOSFET)
//   LCD_RESET = tied to system NRST
//   LED       = PE3 (active-low)

const TICK_HZ: u32 = 10; // TIM2 fires every 100 ms
const LED_PERIOD_TICKS: u32 = 50; // 50 * 100 ms = 5 s between pulses
                                  // pulse width = 1 tick = 100 ms

// Shared between main loop and ISR
static TICKS: AtomicU32 = AtomicU32::new(0);
static G_TIM2: Mutex<RefCell<Option<Timer<TIM2>>>> = Mutex::new(RefCell::new(None));
static G_LED: Mutex<RefCell<Option<PE3<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let mut pwrcfg = pwr.freeze();
    let backup = pwrcfg.backup().unwrap();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(240.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let mut delay = Delay::new(cp.SYST, ccdr.clocks);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // LED PE3 (active-low) — start off
    let mut led = gpioe.pe3.into_push_pull_output();
    led.set_high();

    // Backlight PE10 — active-low: LOW = ON
    let mut bl = gpioe.pe10.into_push_pull_output();
    bl.set_low();

    // CS PE11 — hold LOW (single-device SPI)
    let mut cs = gpioe.pe11.into_push_pull_output();
    cs.set_low();

    let dc = gpioe.pe13.into_push_pull_output();
    let rst = gpioe.pe15.into_push_pull_output();

    // SPI4: SCK=PE12, MOSI=PE14 (AF5)
    let sck = gpioe.pe12.into_alternate::<5>();
    let mosi = gpioe.pe14.into_alternate::<5>();
    let spi = dp.SPI4.spi(
        (sck, spi::NoMiso, mosi),
        spi::MODE_0,
        24.MHz(),
        ccdr.peripheral.SPI4,
        &ccdr.clocks,
    );

    let mut display = ST7735::new(spi, dc, rst, false, true, 160, 128);
    display.init(&mut delay).unwrap();
    display
        .set_orientation(&Orientation::LandscapeSwapped)
        .unwrap();
    display.set_offset(0, 24);
    display.clear(Rgb565::BLACK).unwrap();

    // RTC on LSI.
    // Time is kept in the backup domain across resets/power-off as long as
    // VBAT is held up (CR2032 on the VBAT pad). Backup register 0 stores a
    // magic value; if it's missing we treat this as a cold boot and seed the
    // clock with the build-time default (Asia/Yekaterinburg, UTC+5).
    const RTC_MAGIC: u32 = 0xCAFE_0035;
    let mut rtc = rtc::Rtc::open_or_init(dp.RTC, backup.RTC, RtcClock::Lsi, &ccdr.clocks);
    if rtc.read_backup_reg(0) != RTC_MAGIC {
        // Cold boot — seed with current Asia/Yekaterinburg wall-clock time.
        // Stored as LOCAL time (the RTC is wall-clock; no timezone awareness).
        rtc.set_date_time(
            NaiveDate::from_ymd_opt(2026, 5, 18)
                .unwrap()
                .and_hms_opt(0, 35, 0)
                .unwrap(),
        );
        rtc.write_backup_reg(0, RTC_MAGIC);
    }

    // TIM2 — 100 ms periodic interrupt
    let mut tim2 = dp
        .TIM2
        .timer(TICK_HZ.Hz(), ccdr.peripheral.TIM2, &ccdr.clocks);
    tim2.listen(Event::TimeOut);

    // Move LED + timer into globals so the ISR can reach them
    cortex_m::interrupt::free(|cs| {
        G_LED.borrow(cs).replace(Some(led));
        G_TIM2.borrow(cs).replace(Some(tim2));
    });
    unsafe { pac::NVIC::unmask(interrupt::TIM2) };

    let bg = PrimitiveStyle::with_fill(Rgb565::BLACK);
    let time_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
    let date_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_LIGHT_SKY_BLUE);

    let mut tbuf = [0u8; 8];
    let mut dbuf = [0u8; 10];
    let mut last_drawn_tick = u32::MAX;

    loop {
        let tick = TICKS.load(Ordering::Relaxed);

        // Redraw the screen at most once per tick (every 100 ms)
        if tick != last_drawn_tick {
            last_drawn_tick = tick;

            if let Some(dt) = rtc.date_time() {
                let h = dt.hour() as u8;
                let m = dt.minute() as u8;
                let s = dt.second() as u8;
                let d = dt.day() as u8;
                let mo = dt.month() as u8;
                let y = dt.year();

                Rectangle::new(Point::new(0, 14), Size::new(160, 24))
                    .into_styled(bg)
                    .draw(&mut display)
                    .unwrap();
                Text::with_alignment(
                    fmt_time(&mut tbuf, h, m, s),
                    Point::new(80, 34),
                    time_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();

                Rectangle::new(Point::new(0, 42), Size::new(160, 24))
                    .into_styled(bg)
                    .draw(&mut display)
                    .unwrap();
                Text::with_alignment(
                    fmt_date(&mut dbuf, d, mo, y),
                    Point::new(80, 62),
                    date_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();
            }
        }

        cortex_m::asm::wfi(); // sleep until next interrupt
    }
}

#[interrupt]
fn TIM2() {
    let tick = TICKS.fetch_add(1, Ordering::Relaxed).wrapping_add(1);

    cortex_m::interrupt::free(|cs| {
        if let Some(t) = G_TIM2.borrow(cs).borrow_mut().as_mut() {
            t.clear_irq();
        }
        if let Some(led) = G_LED.borrow(cs).borrow_mut().as_mut() {
            // Pulse LED for one tick (100 ms) every LED_PERIOD_TICKS (= 5 s)
            let phase = tick % LED_PERIOD_TICKS;
            if phase == 0 {
                led.set_low(); // PE3 is active-low → ON
            } else if phase == 1 {
                led.set_high(); // OFF
            }
        }
    });
}

fn fmt_time(buf: &mut [u8; 8], h: u8, m: u8, s: u8) -> &str {
    let d = b"0123456789";
    buf[0] = d[(h / 10) as usize];
    buf[1] = d[(h % 10) as usize];
    buf[2] = b':';
    buf[3] = d[(m / 10) as usize];
    buf[4] = d[(m % 10) as usize];
    buf[5] = b':';
    buf[6] = d[(s / 10) as usize];
    buf[7] = d[(s % 10) as usize];
    core::str::from_utf8(buf).unwrap()
}

fn fmt_date(buf: &mut [u8; 10], d: u8, mo: u8, y: i32) -> &str {
    let dig = b"0123456789";
    let y = y as u32;
    buf[0] = dig[(d / 10) as usize];
    buf[1] = dig[(d % 10) as usize];
    buf[2] = b'.';
    buf[3] = dig[(mo / 10) as usize];
    buf[4] = dig[(mo % 10) as usize];
    buf[5] = b'.';
    buf[6] = dig[(y / 1000 % 10) as usize];
    buf[7] = dig[(y / 100 % 10) as usize];
    buf[8] = dig[(y / 10 % 10) as usize];
    buf[9] = dig[(y % 10) as usize];
    core::str::from_utf8(buf).unwrap()
}
