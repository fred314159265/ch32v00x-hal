#![no_std]
#![no_main]

use hal::gpio::GpioExt;
use hal::rcc::HSEClock;
use hal::rcc::PLLMUL;
use riscv_rt::entry;
// provide implementation for critical-section
use panic_halt as _;
use riscv as _;

use ch32v20x_hal as hal;
use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(48.MHz())
        // .set_defaults()
        // .hse(HSEClock::new(8.MHz(), hal::rcc::HSEClockMode::Oscillator))
        .freeze();

    //assert_eq!(clocks.hclk().to_MHz(), 16);

    // nanoCH32V203
    //let gpioa = peripherals.GPIOA.split();
    //let mut led = gpioa.pa15.into_push_pull_output();

    let gpiob = peripherals.GPIOB.split();
    let mut led = gpiob.pb8.into_push_pull_output();

    // HSI 8MHz
    // 4 opcodes to do a nop sleep here
    let cycle = 8_000_000 / 10;
    loop {
        unsafe { riscv::asm::delay(cycle) }
        led.toggle();
    }
}