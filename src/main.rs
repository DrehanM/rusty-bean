// main.rs
#![no_std]
#![no_main]

extern crate rlibc;

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    panic!("Some panic message");
}

// this is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}