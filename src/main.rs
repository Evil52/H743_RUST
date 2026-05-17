#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32h7xx_hal::{
    delay::Delay,
    pac,
    prelude::*,
    rtc::{self, RtcClock},
    spi,
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
//   LCD_DC    = PE13  (called LCD_WR_RS in schematic)
//   LCD_BL    = PE10  (TIM1_CH2N; ACTIVE-LOW via P-MOSFET — LOW = backlight on)
//   LCD_RESET = tied to system NRST (no GPIO needed; driver gets dummy pin PE15)

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

    // LED PE3 — quick blink so we know main started
    let mut led = gpioe.pe3.into_push_pull_output();
    for _ in 0..6 {
        led.set_high();
        delay.delay_ms(80_u32);
        led.set_low();
        delay.delay_ms(80_u32);
    }

    // Backlight PE10 — active-low (P-MOSFET): LOW = ON
    let mut bl = gpioe.pe10.into_push_pull_output();
    bl.set_low();

    // CS PE11 — hold LOW for the entire session (single-device SPI)
    let mut cs = gpioe.pe11.into_push_pull_output();
    cs.set_low();

    // DC PE13
    let dc = gpioe.pe13.into_push_pull_output();

    // RST — no real LCD reset pin; use unused PE15 as dummy
    let rst = gpioe.pe15.into_push_pull_output();

    // SPI4: SCK=PE12, MOSI=PE14 (AF5)
    let sck  = gpioe.pe12.into_alternate::<5>();
    let mosi = gpioe.pe14.into_alternate::<5>();
    let spi = dp.SPI4.spi(
        (sck, spi::NoMiso, mosi),
        spi::MODE_0,
        24.MHz(),
        ccdr.peripheral.SPI4,
        &ccdr.clocks,
    );

    // ST7735: rgb=false (BGR ordering for this panel), inverted=true
    let mut display = ST7735::new(spi, dc, rst, false, true, 160, 128);
    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::LandscapeSwapped).unwrap();
    // 0.96" 80×160 panel in LandscapeSwapped — column offset = 0, row offset = 24
    display.set_offset(0, 24);
    display.clear(Rgb565::BLACK).unwrap();

    // RTC
    let mut rtc = rtc::Rtc::open_or_init(
        dp.RTC,
        backup.RTC,
        RtcClock::Lsi,
        &ccdr.clocks,
    );
    rtc.set_date_time(
        NaiveDate::from_ymd_opt(2026, 5, 17)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap(),
    );

    let bg         = PrimitiveStyle::with_fill(Rgb565::BLACK);
    let time_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
    let date_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_LIGHT_SKY_BLUE);

    let mut tbuf = [0u8; 8];
    let mut dbuf = [0u8; 10];

    loop {
        if let Some(dt) = rtc.date_time() {
            let h  = dt.hour()   as u8;
            let m  = dt.minute() as u8;
            let s  = dt.second() as u8;
            let d  = dt.day()    as u8;
            let mo = dt.month()  as u8;
            let y  = dt.year();

            // Landscape: 160 × 80. Two lines (20px each), 4px gap → block ≈ 44px.
            // Vertically centered: top margin = (80-44)/2 = 18.
            // Time baseline = 18 + 16 = 34; date baseline = 18 + 24 + 16 = 58.
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

        led.toggle();
        delay.delay_ms(500_u32);
    }
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
    buf[0] = dig[(d  / 10) as usize];
    buf[1] = dig[(d  % 10) as usize];
    buf[2] = b'.';
    buf[3] = dig[(mo / 10) as usize];
    buf[4] = dig[(mo % 10) as usize];
    buf[5] = b'.';
    buf[6] = dig[(y / 1000 % 10) as usize];
    buf[7] = dig[(y /  100 % 10) as usize];
    buf[8] = dig[(y /   10 % 10) as usize];
    buf[9] = dig[(y        % 10) as usize];
    core::str::from_utf8(buf).unwrap()
}
