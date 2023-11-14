#![no_std]
#![no_main]

use ch32v3xx_pac as pac;
use panic_halt as _;
use riscv_rt::entry;
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // 开启 GPIOC 的时钟
    dp.RCC.apb2pcenr.write(|w| w.iopaen().set_bit());
    dp.GPIOA.cfghr.write(|w| unsafe {
        w.cnf15().bits(0b00);
        w.mode15().bits(0b11)
    });

    dp.GPIOC.outdr.write(|w| w.odr15().clear_bit());
    loop {}
}
