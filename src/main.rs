#![no_std]
#![no_main]

// pick a panicking behavior
use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use cortex_m::asm::nop;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");

        for _ in 0..1_000_000 {
            nop();
        }

        panic!("This is an intentional panic.");
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}