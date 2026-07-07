//! Async clock for the WeAct MiniSTM32H7xx — the embassy rewrite of the
//! original bare-metal firmware.
//!
//! Tasks instead of ISRs:
//! - `main`     — owns the RTC + LCD, redraws only what changed;
//! - `led`      — blink pattern selected by the shared MODE atomic;
//! - `button`   — async EXTI wait on K1, advances MODE, time-based debounce.
//!
//! RTC runs from the 32.768 kHz LSE crystal and survives resets in the
//! backup domain; time is (re)seeded only when the calendar has clearly
//! never been set.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU8, Ordering};

use defmt::{info, unwrap, warn};
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::rtc::{DateTime, DayOfWeek, Rtc, RtcConfig};
use embassy_stm32::spi::{self, Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, interrupt};
use embassy_time::Timer;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_hal_bus::spi::ExclusiveDevice;

use weact_h743::{board, display};

bind_interrupts!(struct Irqs {
    EXTI15_10 => exti::InterruptHandler<interrupt::typelevel::EXTI15_10>;
});

/// Written into the RTC once, when the calendar has never been set.
/// (2026-07-07 is a Tuesday.)
const SEED: (u16, u8, u8, DayOfWeek, u8, u8, u8) = (2026, 7, 7, DayOfWeek::Tuesday, 12, 0, 0);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Off = 0,
    Fast = 1,
    Slow = 2,
    Solid = 3,
}

impl Mode {
    fn next(self) -> Mode {
        match self {
            Mode::Off => Mode::Fast,
            Mode::Fast => Mode::Slow,
            Mode::Slow => Mode::Solid,
            Mode::Solid => Mode::Off,
        }
    }
    fn from_u8(v: u8) -> Mode {
        match v & 0b11 {
            0 => Mode::Off,
            1 => Mode::Fast,
            2 => Mode::Slow,
            _ => Mode::Solid,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Mode::Off => "OFF",
            Mode::Fast => "FAST",
            Mode::Slow => "SLOW",
            Mode::Solid => "SOLID",
        }
    }
}

/// The only state shared between tasks; everything else is owned.
static MODE: AtomicU8 = AtomicU8::new(Mode::Off as u8);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(board::config_with_rtc());
    info!("WeAct H743 clock (embassy)");

    spawner.spawn(unwrap!(led(Output::new(p.PE3, Level::Low, Speed::Low))));
    spawner.spawn(unwrap!(button(ExtiInput::new(
        p.PC13,
        p.EXTI13,
        Pull::Down,
        Irqs
    ))));

    // --- RTC: seed only if the calendar was never set. ---
    // `rtc` writes the calendar, `rtc_reader` reads it.
    let (mut rtc, rtc_reader) = Rtc::new(p.RTC, RtcConfig::default());
    let needs_seed = match rtc_reader.now() {
        Ok(dt) => dt.year() < SEED.0,
        Err(_) => true,
    };
    if needs_seed {
        let (y, mo, d, dow, h, mi, s) = SEED;
        match DateTime::from(y, mo, d, dow, h, mi, s, 0) {
            Ok(dt) => {
                if rtc.set_datetime(dt).is_err() {
                    warn!("RTC seed failed");
                }
            }
            Err(_) => warn!("invalid RTC seed constant"),
        }
    }

    // --- LCD ---
    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = Hertz(24_000_000);
    let spi_bus = Spi::new_blocking_txonly(p.SPI4, p.PE12, p.PE14, spi_cfg);
    let cs = Output::new(p.PE11, Level::High, Speed::VeryHigh);
    let spi_dev = unwrap!(ExclusiveDevice::new_no_delay(spi_bus, cs).map_err(|_| ()));
    let dc = Output::new(p.PE13, Level::Low, Speed::VeryHigh);
    let rst = Output::new(p.PE15, Level::High, Speed::Low); // panel resets with NRST
    let mut lcd = unwrap!(display::init_lcd(spi_dev, dc, rst));
    let _backlight = Output::new(p.PE10, Level::Low, Speed::Low); // active low

    let bg = embedded_graphics::primitives::PrimitiveStyle::with_fill(Rgb565::BLACK);
    let mode_style = MonoTextStyle::new(&FONT_10X20, Rgb565::YELLOW);
    let time_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
    let date_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_LIGHT_SKY_BLUE);

    let mut tbuf = [0u8; 8];
    let mut dbuf = [0u8; 10];
    // Track what is on screen so each field redraws only on real change.
    let mut last_mode: Option<Mode> = None;
    let mut last_second: Option<u8> = None;
    let mut last_day: Option<u8> = None;

    loop {
        let mode = Mode::from_u8(MODE.load(Ordering::Relaxed));
        if last_mode != Some(mode) {
            redraw_row(&mut lcd, bg, 0, mode.label(), mode_style);
            last_mode = Some(mode);
        }

        if let Ok(dt) = rtc_reader.now() {
            let s = dt.second();
            if last_second != Some(s) {
                let text = fmt_time(&mut tbuf, dt.hour(), dt.minute(), s);
                redraw_row(&mut lcd, bg, 26, text, time_style);
                last_second = Some(s);
            }

            let d = dt.day();
            if last_day != Some(d) {
                let text = fmt_date(&mut dbuf, d, dt.month(), dt.year());
                redraw_row(&mut lcd, bg, 52, text, date_style);
                last_day = Some(d);
            }
        }

        Timer::after_millis(100).await;
    }
}

/// Clear a 22 px row and draw centered text into it.
fn redraw_row(
    lcd: &mut display::Lcd,
    bg: embedded_graphics::primitives::PrimitiveStyle<Rgb565>,
    y: i32,
    text: &str,
    style: MonoTextStyle<'static, Rgb565>,
) {
    use embedded_graphics::primitives::Rectangle;
    let _ = Rectangle::new(Point::new(0, y), Size::new(160, 22))
        .into_styled(bg)
        .draw(lcd);
    let _ = Text::with_alignment(text, Point::new(80, y + 18), style, Alignment::Center).draw(lcd);
}

/// LED pattern follows MODE. Poll granularity doubles as the blink timer,
/// so a mode change is picked up within one period.
#[embassy_executor::task]
async fn led(mut led: Output<'static>) {
    loop {
        match Mode::from_u8(MODE.load(Ordering::Relaxed)) {
            Mode::Off => {
                led.set_low();
                Timer::after_millis(100).await;
            }
            Mode::Fast => {
                led.toggle();
                Timer::after_millis(100).await;
            }
            Mode::Slow => {
                led.toggle();
                Timer::after_millis(500).await;
            }
            Mode::Solid => {
                led.set_high();
                Timer::after_millis(100).await;
            }
        }
    }
}

/// K1 advances the mode. Debounce = sleep through the bounce window;
/// edges during the sleep are simply not awaited, so they are dropped.
#[embassy_executor::task]
async fn button(mut k1: ExtiInput<'static, Async>) {
    loop {
        k1.wait_for_rising_edge().await;
        let cur = Mode::from_u8(MODE.load(Ordering::Relaxed));
        MODE.store(cur.next() as u8, Ordering::Relaxed);
        info!("mode -> {}", cur.next().label());
        Timer::after_millis(300).await;
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

fn fmt_date(buf: &mut [u8; 10], d: u8, mo: u8, y: u16) -> &str {
    let dig = b"0123456789";
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
