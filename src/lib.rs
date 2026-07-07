//! Shared platform code for the WeAct MiniSTM32H7xx firmwares.
//!
//! Binaries (`src/bin/`) stay thin: task wiring and policy. Everything
//! reusable lives here — board definition, sensor driver, display glue.

#![no_std]

pub mod board;
pub mod display;
pub mod ov2640;
