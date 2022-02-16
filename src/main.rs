#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut flash = peripherals.FLASH.constrain();

    let rcc = peripherals.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .hclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .sysclk(72.mhz())
        .adcclk(12.mhz())
        .freeze(&mut flash.acr);
    // Configure TDO/TRACESWO as PB3 and NJTRST as PB4
    let mut afio =  peripherals.AFIO.constrain();
    let mut timer = Timer::syst(core.SYST, &clocks).start_count_down(10.hz());
    let mut gpioa = peripherals.GPIOA.split();
    let gpiob = peripherals.GPIOB.split();
    let (pa15, _, _) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let mut led = pa15.into_push_pull_output(&mut gpioa.crh);

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
