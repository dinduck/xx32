#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let pac = ch32v3xx_pac::Peripherals::take().unwrap();
    let gpioa = &pac.GPIOA;
    gpioa.outdr.write(|w| w.odr0().set_bit());
    loop {}
}
