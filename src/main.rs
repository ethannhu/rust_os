#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(prototype::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use prototype::{hlt_loop, println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world {}!", "Blanca");

    prototype::init();
    #[cfg(test)]
    test_main();

    println!("It Survived!");
    hlt_loop()
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("panic: {}", _info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    prototype::test_panic_handler(info)
}
