#![no_std]
#![no_main]

extern crate panic_semihosting;

//use stlinky::{stlinky_buffer, sprintln};

use nb::block;
use cortex_m_rt::entry;
use stm32f1xx_hal::{prelude::*, stm32, timer::Timer};
use usb_device::prelude::*;
use stm32f103xx_usb::UsbBus;

mod midi {
    use usb_device::Result;
    use usb_device::class_prelude::*;

    pub struct MidiClass<'a, B: UsbBus> {
        audio_if: InterfaceNumber,
        midi_if: InterfaceNumber,
        out_ep: EndpointOut<'a, B>,
        in_ep: EndpointIn<'a, B>,
    }

    impl<B: UsbBus> MidiClass<'_, B> {
        pub fn new(alloc: &UsbBusAllocator<B>) -> MidiClass<'_, B> {
            MidiClass {
                audio_if: alloc.interface(),
                midi_if: alloc.interface(),
                out_ep: alloc.bulk(64),
                in_ep: alloc.bulk(64),
            }
        }

        pub fn note_off(&self, chan: u8, key: u8, vel: u8) -> Result<usize> {
            // I have no idea why the "virtual cable" must be number 0 and not one of the jack IDs
            // but only 0 seemed to work
            self.in_ep.write(&[0x08, 0x80 | (chan & 0x0f), key & 0x7f, vel & 0x7f])
        }

        pub fn note_on(&self, chan: u8, key: u8, vel: u8) -> Result<usize> {
            self.in_ep.write(&[0x09, 0x90 | (chan & 0x0f), key & 0x7f, vel & 0x7f])
        }
    }

    impl<B: UsbBus> UsbClass<B> for MidiClass<'_, B> {
        fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
            writer.interface(self.audio_if, 0x01, 0x01, 0x00)?; // Interface 0
            writer.write(0x24, &[0x01, 0x00, 0x01, 0x09, 0x00, 0x01, 0x01])?; // CS Interface (audio)

            writer.interface(self.midi_if, 0x01, 0x03, 0x00)?; // Interface 1
            writer.write(0x24, &[0x01, 0x00, 0x01, 0x2e, 0x00])?; // CS Interface (midi)
            writer.write(0x24, &[0x02, 0x01, 0x01, 0x00])?; // IN Jack 1 (emb)
            writer.write(0x24, &[0x03, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00])?; // OUT Jack 2 (emb)

            writer.endpoint(&self.out_ep)?;
            writer.write(0x25, &[0x01, 0x01, 0x01])?; // CS EP IN Jack

            writer.endpoint(&self.in_ep)?;
            writer.write(0x25, &[0x01, 0x01, 0x02])?; // CS EP OUT Jack

            Ok(())
        }
    }
}

//stlinky_buffer!(STDOUT, 1024);

#[entry]
fn main() -> ! {
    //stlinky::set_stdout_at(unsafe { &mut STDOUT });

    //sprintln!("starting up!");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid(), "usb clocks not valid");

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let usb_bus = UsbBus::usb_with_reset(dp.USB,
        &mut rcc.apb1, &clocks, &mut gpioa.crh, gpioa.pa12);

    let mut midi = midi::MidiClass::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27de))
        .manufacturer("Fake company")
        .product("MIDI Device")
        .serial_number("TEST")
        .build();

    usb_dev.force_reset().expect("reset failed");

    let mut timer = Timer::syst(cp.SYST, 1000.hz(), clocks);

    let notes = &[60, 62, 64, 65, 67, 69, 71, 72];
    let mut note_iter = notes.iter().cycle();
    let mut note = *note_iter.next().unwrap();

    let mut ms = 0;
    loop {
        while usb_dev.poll(&mut [&mut midi]) { }

        if usb_dev.state() == UsbDeviceState::Configured {
            // Excuse the super crude sequencer

            if ms == 200 {
                if midi.note_on(0, note, 64).is_ok() {
                    led.set_low();
                    ms += 1;
                }
            } else if ms == 400 {
                if midi.note_off(0, note, 0).is_ok() {
                    led.set_high();
                    ms = 0;
                    note = *note_iter.next().unwrap();
                }
            } else {
                ms += 1;
            }

            block!(timer.wait()).unwrap();
        }
    }
}