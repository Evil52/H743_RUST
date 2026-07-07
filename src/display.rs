//! Viewfinder on the onboard 0.96" ST7735 (160x80 visible pixels).
//!
//! The camera streams 320x160 RGB565 frames; [`draw_frame`] downsamples
//! 2:1 (nearest neighbor) and blits the result over SPI. The panel is
//! also the firmware's "error console": a sensor that fails to probe is
//! reported on screen, not just over RTT.

use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Blocking;
use embassy_stm32::spi::{mode::Master, Spi};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use st7735_lcd::ST7735;

pub const LCD_W: usize = 160;
pub const LCD_H: usize = 80;

/// Camera frame: 2x the viewfinder in both axes, RGB565.
pub const FRAME_W: usize = LCD_W * 2;
pub const FRAME_H: usize = LCD_H * 2;
/// DCMI captures into word-sized buffers: 2 bytes/pixel, 2 pixels/word.
pub const FRAME_WORDS: usize = FRAME_W * FRAME_H / 2;

pub type SpiDev = ExclusiveDevice<Spi<'static, Blocking, Master>, Output<'static>, NoDelay>;
pub type Lcd = ST7735<SpiDev, Output<'static>, Output<'static>>;

/// Bring the panel up the way this board needs it: BGR subpixel order,
/// inverted colors, landscape, 24-row GRAM offset (the 0.96" glass shows
/// 160x80 of the controller's memory).
// `()` is the error type of the st7735-lcd driver itself — nothing to add to it.
#[allow(clippy::result_unit_err)]
pub fn init_lcd(spi: SpiDev, dc: Output<'static>, rst: Output<'static>) -> Result<Lcd, ()> {
    let mut lcd = ST7735::new(spi, dc, rst, false, true, 160, 128);
    lcd.init(&mut embassy_time::Delay)?;
    lcd.set_orientation(&st7735_lcd::Orientation::LandscapeSwapped)?;
    lcd.set_offset(0, 24);
    lcd.clear(Rgb565::BLACK).map_err(|_| ())?;
    Ok(lcd)
}

/// Downsample 320x160 -> 160x80 and push to the panel.
///
/// Sampling every second pixel of every second row means every source
/// index is even, i.e. always the low half of a DCMI word (the sensor
/// sends RGB565 low-byte-first, so pixels are little-endian in memory).
pub fn draw_frame(lcd: &mut Lcd, frame: &[u32; FRAME_WORDS]) {
    let pixels = (0..LCD_H).flat_map(|y| {
        let row_px = (y * 2) * FRAME_W;
        (0..LCD_W).map(move |x| (frame[(row_px + x * 2) / 2] & 0xFFFF) as u16)
    });
    let _ = lcd.set_pixels_buffered(0, 0, (LCD_W - 1) as u16, (LCD_H - 1) as u16, pixels);
}

/// Big red message, centered. For states that need a human (no sensor,
/// wrong sensor) — visible without a debug probe attached.
pub fn show_error(lcd: &mut Lcd, msg: &str) {
    let _ = lcd.clear(Rgb565::BLACK);
    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::RED);
    let _ = Text::with_alignment(
        msg,
        Point::new(LCD_W as i32 / 2, LCD_H as i32 / 2 + 7),
        style,
        Alignment::Center,
    )
    .draw(lcd);
}
