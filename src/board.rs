//! Board support for the WeAct Studio MiniSTM32H7xx (STM32H743VIT6).
//!
//! Everything that is a property of *this PCB* — clock tree, pin mapping —
//! lives here, so the rest of the firmware only talks in terms of roles
//! (camera bus, viewfinder, heartbeat LED).
//!
//! Pin map (schematic V1.2):
//!
//! | Role                | Pin  | Peripheral       |
//! |---------------------|------|------------------|
//! | Heartbeat LED       | PE3  | GPIO (active hi) |
//! | User button K1      | PC13 | GPIO/EXTI        |
//! | Camera XCLK         | PA8  | MCO1             |
//! | Camera SCCB SCL/SDA | PB8/PB9 | I2C1          |
//! | DCMI D0..D7         | PC6 PC7 PE0 PE1 PE4 PD3 PE5 PE6 | DCMI |
//! | DCMI VSYNC/HSYNC/PIXCLK | PB7 / PA4 / PA6 | DCMI |
//! | LCD SCK/MOSI        | PE12/PE14 | SPI4        |
//! | LCD CS / DC         | PE11/PE13 | GPIO        |
//! | LCD backlight       | PE10 | GPIO (active lo via P-MOSFET) |

use embassy_stm32::rcc;
use embassy_stm32::time::Hertz;
use embassy_stm32::Config;

/// 25 MHz crystal (X1) on the WeAct board.
pub const HSE_FREQ: Hertz = Hertz(25_000_000);

/// XCLK fed to the OV2640 = HSE / 2 = 12.5 MHz.
/// OpenMV drives this sensor at 12 MHz; the sensor's internal PLL
/// (CLKRC doubler) generates the pixel clock from it.
pub const XCLK_PRESCALER: rcc::McoPrescaler = rcc::McoPrescaler::DIV2;

/// Clock tree: HSE 25 MHz -> PLL1 -> 400 MHz sysclk, 200 MHz AHB, 100 MHz APB.
pub fn config() -> Config {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: HSE_FREQ,
            mode: HseMode::Oscillator,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV5,  // 25 / 5 = 5 MHz PLL input
            mul: PllMul::MUL160,      // 5 * 160 = 800 MHz VCO
            fracn: None,
            divp: Some(PllDiv::DIV2), // 400 MHz sysclk
            divq: Some(PllDiv::DIV8), // 100 MHz kernel clock
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 MHz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 MHz
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.apb3_pre = APBPrescaler::DIV2;
        config.rcc.apb4_pre = APBPrescaler::DIV2;
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    config
}

/// Same clock tree plus the RTC fed by the 32.768 kHz crystal (X2).
/// Split out because waiting for the LSE to start takes a moment and
/// only the clock firmware needs it; the backup domain keeps the LSE
/// running across resets afterwards.
pub fn config_with_rtc() -> Config {
    let mut config = config();
    config.rcc.ls = rcc::LsConfig::default_lse();
    config
}
