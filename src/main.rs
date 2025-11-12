#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("Hello Macro!");
    panic!("Crash and burn!");
    loop {}
}