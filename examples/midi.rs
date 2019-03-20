#![no_std]
#![no_main]

/// CDC-ACM serial port example using polling in a busy loop.
extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f1xx_hal::{prelude::*, stm32};

use cortex_m_semihosting::hprintln;
use stm32f103xx_usb::UsbBus;
use usb_device::prelude::*;

//mod cdc_acm;
mod audio_midi;

#[entry]
#[inline(never)]
fn main() -> ! {
    hprintln!("before clock").unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());
    hprintln!("after clock").unwrap();

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let usb_bus = UsbBus::usb_with_reset(
        dp.USB,
        &mut rcc.apb1,
        &clocks,
        &mut gpioa.crh,
        gpioa.pa12,
    );

    let mut midi = audio_midi::MsInterface::new(&usb_bus);

    let mut usb_dev =
        UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
            .manufacturer("Per Lindgren")
            .product("USB MIDI")
            .serial_number("0123")
            .device_class(audio_midi::USB_CLASS_AUDIO)
            .build();

    usb_dev.force_reset().expect("reset failed");

    loop {
        if !usb_dev.poll(&mut [&mut midi]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match midi.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                midi.write(&buf[0..count]).ok();
            }
            _ => {}
        }
    }
}
