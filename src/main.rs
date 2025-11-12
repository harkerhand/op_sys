#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(op_sys::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use op_sys::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    op_sys::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    op_sys::test_panic_handler(info)
}