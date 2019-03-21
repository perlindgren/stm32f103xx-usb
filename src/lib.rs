//! USB peripheral driver for STM32F103 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]

//extern crate bare_metal;
//extern crate cortex_m;
//extern crate stm32f103xx;
//extern crate stm32f103xx_hal;
//extern crate vcell;
//extern crate usb_device;

mod endpoint;

mod atomic_mutex;

/// USB peripheral driver.
pub mod bus;

pub use crate::bus::UsbBus;
