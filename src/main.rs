#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32h7xx_hal::{
    delay::Delay,
    gpio::{gpioc::PC13, gpioe::PE3, Edge, ExtiPin, Input, Output, PushPull},
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

// WeAct STM32H743VIT6 Mini pinout:
//   SPI4: SCK=PE12, MOSI=PE14   LCD_CS=PE11   LCD_DC=PE13
//   LCD_BL=PE10 (active-low via P-MOSFET)     LCD_RESET tied to NRST
//   LED=PE3 (active-high via NPN)             K1=PC13 (active-high)

const TICK_HZ: u32 = 10;
// 500 ms half-period for SLOW blink at a 10 Hz tick.
const SLOW_HALF_TICKS: u32 = TICK_HZ / 2;
// Debounce lockout for the K1 button (~300 ms).
const DEBOUNCE_TICKS: u32 = 3;
// Bump this whenever you change the seeded date/time below — a mismatch with
// the value stored in backup register 0 forces a re-seed on next boot.
const RTC_MAGIC: u32 = 0xCAFE_0036;

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
            Mode::Off => "OFF  ",
            Mode::Fast => "FAST ",
            Mode::Slow => "SLOW ",
            Mode::Solid => "SOLID",
        }
    }
}

// Shared state between main and ISRs.
static TICKS: AtomicU32 = AtomicU32::new(0);
static MODE: AtomicU8 = AtomicU8::new(Mode::Off as u8);
// Next tick at which a button press is allowed (debounce lockout).
static DEBOUNCE_UNTIL: AtomicU32 = AtomicU32::new(0);

static G_TIM2: Mutex<RefCell<Option<Timer<TIM2>>>> = Mutex::new(RefCell::new(None));
static G_LED: Mutex<RefCell<Option<PE3<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static G_BTN: Mutex<RefCell<Option<PC13<Input>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let mut pwrcfg = pwr.freeze();
    let backup = pwrcfg.backup().unwrap();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(240.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let mut delay = Delay::new(cp.SYST, ccdr.clocks);

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut led = gpioe.pe3.into_push_pull_output();
    led.set_low();

    let mut btn = gpioc.pc13.into_pull_down_input();
    let mut syscfg = dp.SYSCFG;
    btn.make_interrupt_source(&mut syscfg);
    btn.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    btn.enable_interrupt(&mut dp.EXTI);

    // Backlight on (active-low through P-MOSFET).
    let mut bl = gpioe.pe10.into_push_pull_output();
    bl.set_low();

    // Single device on SPI4: drive CS low for the entire session.
    let mut cs_pin = gpioe.pe11.into_push_pull_output();
    cs_pin.set_low();

    let dc = gpioe.pe13.into_push_pull_output();
    // No physical LCD reset line (panel resets with NRST). Driver still needs
    // an OutputPin, so we hand it an unused GPIO.
    let rst = gpioe.pe15.into_push_pull_output();

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

    // RTC on LSE (32.768 kHz crystal X2 on the WeAct board — ±20 ppm,
    // seconds-per-day accuracy vs. minutes/day on LSI).
    let mut rtc = rtc::Rtc::open_or_init(
        dp.RTC,
        backup.RTC,
        RtcClock::Lse {
            freq: 32_768.Hz(),
            bypass: false,
            css: false,
        },
        &ccdr.clocks,
    );
    if rtc.read_backup_reg(0) != RTC_MAGIC {
        rtc.set_date_time(
            NaiveDate::from_ymd_opt(2026, 5, 18)
                .unwrap()
                .and_hms_opt(0, 35, 0)
                .unwrap(),
        );
        rtc.write_backup_reg(0, RTC_MAGIC);
    }

    let mut tim2 = dp
        .TIM2
        .timer(TICK_HZ.Hz(), ccdr.peripheral.TIM2, &ccdr.clocks);
    tim2.listen(Event::TimeOut);

    cortex_m::interrupt::free(|cs| {
        G_LED.borrow(cs).replace(Some(led));
        G_TIM2.borrow(cs).replace(Some(tim2));
        G_BTN.borrow(cs).replace(Some(btn));
    });
    // SAFETY: `NVIC::unmask` is `unsafe` because enabling an IRQ before its
    // handler is ready to run (peripheral configured, shared state published)
    // can break mask-based critical sections elsewhere in the program. Here
    // both invariants are satisfied:
    //   1. TIM2 and PC13/EXTI are fully configured above.
    //   2. The globals G_LED, G_TIM2, G_BTN have already been populated
    //      inside the `interrupt::free` block right above this call, so the
    //      first time TIM2/EXTI15_10 fires the ISRs see initialized state.
    // There is no safe API in stm32h7xx-hal to enable an NVIC line, so this
    // is the only way to arm the interrupts.
    unsafe {
        pac::NVIC::unmask(interrupt::TIM2);
        pac::NVIC::unmask(interrupt::EXTI15_10);
    }

    let bg = PrimitiveStyle::with_fill(Rgb565::BLACK);
    let time_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
    let date_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_LIGHT_SKY_BLUE);
    let mode_style = MonoTextStyle::new(&FONT_10X20, Rgb565::YELLOW);

    let mut tbuf = [0u8; 8];
    let mut dbuf = [0u8; 10];
    // Track what is currently shown so we only redraw on real change.
    let mut last_second: Option<u8> = None;
    let mut last_day: Option<u8> = None;
    let mut last_mode: Option<Mode> = None;

    loop {
        let mode = Mode::from_u8(MODE.load(Ordering::Relaxed));

        if let Some(dt) = rtc.date_time() {
            let s = dt.second() as u8;
            let d = dt.day() as u8;

            if last_mode != Some(mode) {
                Rectangle::new(Point::new(0, 0), Size::new(160, 22))
                    .into_styled(bg)
                    .draw(&mut display)
                    .unwrap();
                Text::with_alignment(
                    mode.label(),
                    Point::new(80, 18),
                    mode_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();
                last_mode = Some(mode);
            }

            if last_second != Some(s) {
                let h = dt.hour() as u8;
                let m = dt.minute() as u8;
                Rectangle::new(Point::new(0, 26), Size::new(160, 22))
                    .into_styled(bg)
                    .draw(&mut display)
                    .unwrap();
                Text::with_alignment(
                    fmt_time(&mut tbuf, h, m, s),
                    Point::new(80, 44),
                    time_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();
                last_second = Some(s);
            }

            if last_day != Some(d) {
                let mo = dt.month() as u8;
                let y = dt.year();
                Rectangle::new(Point::new(0, 52), Size::new(160, 22))
                    .into_styled(bg)
                    .draw(&mut display)
                    .unwrap();
                Text::with_alignment(
                    fmt_date(&mut dbuf, d, mo, y),
                    Point::new(80, 70),
                    date_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();
                last_day = Some(d);
            }
        }

        cortex_m::asm::wfi();
    }
}

// 10 Hz tick: drives the LED pattern.
#[interrupt]
fn TIM2() {
    let tick = TICKS.fetch_add(1, Ordering::Relaxed).wrapping_add(1);

    cortex_m::interrupt::free(|cs| {
        if let Some(t) = G_TIM2.borrow(cs).borrow_mut().as_mut() {
            t.clear_irq();
        }
        if let Some(led) = G_LED.borrow(cs).borrow_mut().as_mut() {
            match Mode::from_u8(MODE.load(Ordering::Relaxed)) {
                Mode::Off => led.set_low(),
                // 100 ms on / 100 ms off — closest to 50 ms at a 10 Hz tick.
                Mode::Fast => {
                    if tick & 1 == 0 { led.set_high() } else { led.set_low() }
                }
                // 500 ms on / 500 ms off — toggle every SLOW_HALF_TICKS ticks.
                Mode::Slow => {
                    if (tick / SLOW_HALF_TICKS) & 1 == 0 {
                        led.set_high()
                    } else {
                        led.set_low()
                    }
                }
                Mode::Solid => led.set_high(),
            }
        }
    });
}

// K1 press → advance MODE. Debounced via a tick-based lockout.
#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        if let Some(b) = G_BTN.borrow(cs).borrow_mut().as_mut() {
            if b.check_interrupt() {
                b.clear_interrupt_pending_bit();

                let now = TICKS.load(Ordering::Relaxed);
                let until = DEBOUNCE_UNTIL.load(Ordering::Relaxed);
                // Wrapping-aware "now >= until" check.
                if now.wrapping_sub(until) < u32::MAX / 2 {
                    let cur = Mode::from_u8(MODE.load(Ordering::Relaxed));
                    MODE.store(cur.next() as u8, Ordering::Relaxed);
                    DEBOUNCE_UNTIL.store(now.wrapping_add(DEBOUNCE_TICKS), Ordering::Relaxed);
                }
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
