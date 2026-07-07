//! OV2640 camera sensor driver (SCCB over I2C, 8-bit parallel DVP out).
//!
//! Hardware-independent: generic over any [`embedded_hal_async::i2c::I2c`],
//! so it can be reused on another board or bus unchanged.
//!
//! The register recipes are transcribed from the OpenMV driver
//! (`drivers/sensors/ov2640.c`, MIT license) with the symbolic constants
//! resolved to raw bytes. The sensor is configured with the SVGA (800x600)
//! sensor timing and the DSP's DCW scaler produces the requested output
//! size — the same strategy OpenMV uses for every size up to 800x600.

use embedded_hal_async::i2c::I2c;

/// 7-bit SCCB address of the OV2640.
pub const ADDR: u8 = 0x30;

/// Register 0xFF selects the active bank; the same address means
/// different things in each bank.
const BANK_SEL: u8 = 0xFF;
const BANK_DSP: u8 = 0x00;
const BANK_SENSOR: u8 = 0x01;

// Sensor bank registers used outside the tables.
const REG_COM7: u8 = 0x12;
const COM7_SRST: u8 = 0x80;
const COM7_COLOR_BAR: u8 = 0x02;
const REG_PID: u8 = 0x0A;
const REG_VER: u8 = 0x0B;

// DSP bank registers used by the size/format recipes.
const REG_CTRLI: u8 = 0x50;
const REG_HSIZE: u8 = 0x51;
const REG_VSIZE: u8 = 0x52;
const REG_XOFFL: u8 = 0x53;
const REG_YOFFL: u8 = 0x54;
const REG_VHYX: u8 = 0x55;
const REG_TEST: u8 = 0x57;
const REG_ZMOW: u8 = 0x5A;
const REG_ZMOH: u8 = 0x5B;
const REG_ZMHH: u8 = 0x5C;
const REG_DVP_SP: u8 = 0xD3;
const REG_RESET: u8 = 0xE0;

const SENSOR_WIDTH: u16 = 800; // SVGA timing is the base for all sizes we use
const SENSOR_HEIGHT: u16 = 600;

/// Baseline configuration (Linux driver heritage + OpenMV tweaks):
/// clocking, AGC/AEC, banding filter, lens/color correction matrices,
/// gamma, AWB. Written once after soft reset.
const DEFAULT_REGS: &[(u8, u8)] = &[
    (BANK_SEL, BANK_DSP),
    (0x2C, 0xFF),
    (0x2E, 0xDF),
    (BANK_SEL, BANK_SENSOR),
    (0x3C, 0x32),
    (0x11, 0x82), // CLKRC: PLL doubler on
    (0x09, 0x02), // COM2: 3x output drive
    (0x04, 0xF8), // REG04: h-flip + v-flip + VREF/HREF bits
    (0x13, 0xE5), // COM8: banding filter + AGC + AEC on
    (0x14, 0x48), // COM9: AGC ceiling 8x
    (0x2C, 0x0C),
    (0x33, 0x78),
    (0x3A, 0x33),
    (0x3B, 0xFB),
    (0x3E, 0x00),
    (0x43, 0x11),
    (0x16, 0x10),
    (0x39, 0x02),
    (0x35, 0x88),
    (0x22, 0x0A),
    (0x37, 0x40),
    (0x23, 0x00),
    (0x34, 0xA0), // ARCOM2
    (0x06, 0x02),
    (0x06, 0x88),
    (0x07, 0xC0),
    (0x0D, 0xB7),
    (0x0E, 0x01),
    (0x4C, 0x00),
    (0x4A, 0x81),
    (0x21, 0x99),
    (0x24, 0x40), // AEW
    (0x25, 0x38), // AEB
    (0x26, 0x82), // VV: AGC thresholds
    (0x5C, 0x00),
    (0x63, 0x00),
    (0x46, 0x22), // FLL
    (0x0C, 0x3A), // COM3: auto banding
    (0x5D, 0x55),
    (0x5E, 0x7D),
    (0x5F, 0x7D),
    (0x60, 0x55),
    (0x61, 0x70), // HISTO_LOW
    (0x62, 0x80), // HISTO_HIGH
    (0x7C, 0x05),
    (0x20, 0x80),
    (0x28, 0x30),
    (0x6C, 0x00),
    (0x6D, 0x80),
    (0x6E, 0x00),
    (0x70, 0x02),
    (0x71, 0x94),
    (0x73, 0xC1),
    (0x3D, 0x34),
    (0x12, 0x04), // COM7: UXGA + zoom enable (overridden by set_output_size)
    (0x5A, 0x57),
    (0x4E, 0x00), // COM25
    (0x4F, 0xBB), // BD50
    (0x50, 0x9C), // BD60
    (BANK_SEL, BANK_DSP),
    (0xE5, 0x7F),
    (0xF9, 0xC0), // MC_BIST: reset microcontroller, boot-ROM select
    (0x41, 0x24),
    (0xE0, 0x14), // RESET: JPEG + DVP
    (0x76, 0xFF),
    (0x33, 0xA0),
    (0x42, 0x20),
    (0x43, 0x18),
    (0x4C, 0x00),
    (0x87, 0xD0), // CTRL3: black/white pixel correction
    (0x88, 0x3F),
    (0xD7, 0x03),
    (0xD9, 0x10),
    (0xD3, 0x82), // R_DVP_SP: auto PCLK mode
    (0xC8, 0x08),
    (0xC9, 0x80),
    (0x7C, 0x00), // BPADDR/BPDATA: SDE indirect registers
    (0x7D, 0x00),
    (0x7C, 0x03),
    (0x7D, 0x48),
    (0x7D, 0x48),
    (0x7C, 0x08),
    (0x7D, 0x20),
    (0x7D, 0x10),
    (0x7D, 0x0E),
    (0x90, 0x00), // gamma curve
    (0x91, 0x0E),
    (0x91, 0x1A),
    (0x91, 0x31),
    (0x91, 0x5A),
    (0x91, 0x69),
    (0x91, 0x75),
    (0x91, 0x7E),
    (0x91, 0x88),
    (0x91, 0x8F),
    (0x91, 0x96),
    (0x91, 0xA3),
    (0x91, 0xAF),
    (0x91, 0xC4),
    (0x91, 0xD7),
    (0x91, 0xE8),
    (0x91, 0x20),
    (0x92, 0x00),
    (0x93, 0x06),
    (0x93, 0xE3),
    (0x93, 0x03),
    (0x93, 0x03),
    (0x93, 0x00),
    (0x93, 0x02),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x96, 0x00),
    (0x97, 0x08),
    (0x97, 0x19),
    (0x97, 0x02),
    (0x97, 0x0C),
    (0x97, 0x24),
    (0x97, 0x30),
    (0x97, 0x28),
    (0x97, 0x26),
    (0x97, 0x02),
    (0x97, 0x98),
    (0x97, 0x80),
    (0x97, 0x00),
    (0x97, 0x00),
    (0xA4, 0x00),
    (0xA8, 0x00),
    (0xC5, 0x11),
    (0xC6, 0x51),
    (0xBF, 0x80),
    (0xC7, 0x10), // simple AWB
    (0xB6, 0x66), // color matrix
    (0xB8, 0xA5),
    (0xB7, 0x64),
    (0xB9, 0x7C),
    (0xB3, 0xAF),
    (0xB4, 0x97),
    (0xB5, 0xFF),
    (0xB0, 0xC5),
    (0xB1, 0x94),
    (0xB2, 0x0F),
    (0xC4, 0x5C),
    (0xA6, 0x00), // lens correction
    (0xA7, 0x20),
    (0xA7, 0xD8),
    (0xA7, 0x1B),
    (0xA7, 0x31),
    (0xA7, 0x00),
    (0xA7, 0x18),
    (0xA7, 0x20),
    (0xA7, 0xD8),
    (0xA7, 0x19),
    (0xA7, 0x31),
    (0xA7, 0x00),
    (0xA7, 0x18),
    (0xA7, 0x20),
    (0xA7, 0xD8),
    (0xA7, 0x19),
    (0xA7, 0x31),
    (0xA7, 0x00),
    (0xA7, 0x18),
    (0x7F, 0x00),
    (0xE5, 0x1F),
    (0xE1, 0x77),
    (0xDD, 0x7F),
    (0xC2, 0x0E), // CTRL0: YUV422 + YUV + RGB enable
    // OpenMV custom:
    (BANK_SEL, BANK_SENSOR),
    (0x0F, 0x4B),
    (0x03, 0x8F), // COM1
];

/// Switch the sensor timing to SVGA (800x600) and reset the DVP path.
const SVGA_REGS: &[(u8, u8)] = &[
    (BANK_SEL, BANK_SENSOR),
    (0x12, 0x40), // COM7: SVGA
    (0x03, 0x8A), // COM1
    (0x17, 0x11), // HSTART
    (0x18, 0x43), // HSTOP
    (0x19, 0x01), // VSTART (0x01 avoids garbage pixels, per OpenMV)
    (0x1A, 0x97), // VSTOP
    (0x32, 0x09), // REG32
    (BANK_SEL, BANK_DSP),
    (0xE0, 0x04), // RESET: DVP
    (0x8C, 0x00), // SIZEL for 800x600
    (0xC0, 0x64), // HSIZE8 = 800 >> 3
    (0xC1, 0x4B), // VSIZE8 = 600 >> 3
    (0x86, 0x3D), // CTRL2: DCW + SDE + UV avg/adj + CMX
];

/// DSP output format: RGB565, low byte first on the DVP bus.
const RGB565_REGS: &[(u8, u8)] = &[
    (BANK_SEL, BANK_DSP),
    (0x05, 0x00), // R_BYPASS: DSP enabled
    (0xDA, 0x09), // IMAGE_MODE: RGB565 + LSB first
    (0xD7, 0x03),
    (0xE0, 0x00), // RESET: none (re-enable everything)
    (0x05, 0x00),
];

#[derive(Debug, defmt::Format, PartialEq, Eq)]
pub enum Error<E> {
    /// SCCB transaction failed (wiring, pull-ups, address).
    I2c(E),
    /// A device answered but it is not an OV2640.
    WrongId { pid: u8, ver: u8 },
    /// Requested output size is not achievable from SVGA timing.
    UnsupportedSize { width: u16, height: u16 },
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::I2c(e)
    }
}

pub struct Ov2640<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> Ov2640<I2C> {
    /// Probe the sensor and verify its product ID (PID 0x26).
    pub async fn probe(i2c: I2C) -> Result<Self, Error<I2C::Error>> {
        let mut cam = Self { i2c };
        cam.write(BANK_SEL, BANK_SENSOR).await?;
        let pid = cam.read(REG_PID).await?;
        let ver = cam.read(REG_VER).await?;
        if pid != 0x26 {
            return Err(Error::WrongId { pid, ver });
        }
        defmt::info!("OV2640 found, PID=0x{:02x} VER=0x{:02x}", pid, ver);
        Ok(cam)
    }

    /// Soft-reset and load the baseline configuration.
    /// Takes ~400 ms: the sensor needs time after reset and after the
    /// register avalanche before the image pipeline is stable.
    pub async fn init(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write(BANK_SEL, BANK_SENSOR).await?;
        self.write(REG_COM7, COM7_SRST).await?;
        embassy_time::Timer::after_millis(10).await;
        self.write_all(DEFAULT_REGS).await?;
        embassy_time::Timer::after_millis(300).await;
        Ok(())
    }

    pub async fn set_pixel_format_rgb565(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_all(RGB565_REGS).await
    }

    /// Set the DVP output size (both dimensions multiples of 4, at most
    /// 800x600). The DSP crops a centered window from the SVGA frame and
    /// downscales it by a power of two, exactly like OpenMV's
    /// `set_framesize`, so any aspect ratio works.
    pub async fn set_output_size(&mut self, width: u16, height: u16) -> Result<(), Error<I2C::Error>> {
        if width % 4 != 0 || height % 4 != 0 || width > SENSOR_WIDTH || height > SENSOR_HEIGHT {
            return Err(Error::UnsupportedSize { width, height });
        }

        self.write_all(SVGA_REGS).await?;

        // Largest power-of-two divider (max 8) that still leaves a
        // window no larger than the sensor.
        let max_div = (SENSOR_WIDTH / width).min(SENSOR_HEIGHT / height);
        let log_div = (15 - (max_div as u16).leading_zeros() as u16).min(3);
        let div = 1u16 << log_div;
        let w_win = width * div;
        let h_win = height * div;
        let x_off = (SENSOR_WIDTH - w_win) / 2;
        let y_off = (SENSOR_HEIGHT - h_win) / 2;

        let ctrli = 0x80 | ((log_div as u8 & 0x3) << 3) | (log_div as u8 & 0x3);
        let vhyx = (((h_win >> 10) & 1) << 7
            | ((y_off >> 8) & 3) << 4
            | ((w_win >> 10) & 1) << 3
            | (x_off >> 8) & 3) as u8;
        let zmhh = (((height >> 10) & 1) << 2 | (width >> 10) & 3) as u8;

        let size_regs: [(u8, u8); 12] = [
            (REG_CTRLI, ctrli),
            (REG_HSIZE, (w_win >> 2) as u8),
            (REG_VSIZE, (h_win >> 2) as u8),
            (REG_XOFFL, x_off as u8),
            (REG_YOFFL, y_off as u8),
            (REG_VHYX, vhyx),
            (REG_TEST, (((w_win >> 11) & 1) << 7) as u8),
            (REG_ZMOW, (width >> 2) as u8),
            (REG_ZMOH, (height >> 2) as u8),
            (REG_ZMHH, zmhh),
            (REG_DVP_SP, div as u8),
            (REG_RESET, 0x00),
        ];
        // Already in the DSP bank after SVGA_REGS.
        self.write_all(&size_regs).await
    }

    /// Enable/disable the sensor's built-in color-bar test pattern.
    /// Invaluable for bring-up: rules out optics, exposure and focus.
    pub async fn set_colorbar(&mut self, enable: bool) -> Result<(), Error<I2C::Error>> {
        self.write(BANK_SEL, BANK_SENSOR).await?;
        let com7 = self.read(REG_COM7).await?;
        let com7 = if enable {
            com7 | COM7_COLOR_BAR
        } else {
            com7 & !COM7_COLOR_BAR
        };
        self.write(REG_COM7, com7).await
    }

    async fn write_all(&mut self, regs: &[(u8, u8)]) -> Result<(), Error<I2C::Error>> {
        for &(reg, val) in regs {
            self.write(reg, val).await?;
        }
        Ok(())
    }

    async fn write(&mut self, reg: u8, val: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(ADDR, &[reg, val]).await?;
        Ok(())
    }

    async fn read(&mut self, reg: u8) -> Result<u8, Error<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(ADDR, &[reg], &mut buf).await?;
        Ok(buf[0])
    }
}
