#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(op_sys::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use op_sys::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    op_sys::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}