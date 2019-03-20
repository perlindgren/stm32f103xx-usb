#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
extern crate stm32f1xx_hal;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    loop {}
}
