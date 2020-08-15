// main.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustybean::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rustybean::println;
use core::panic::PanicInfo;

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rustybean::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    };

    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("No crash!");

    loop {}
}


// this is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustybean::test_panic_handler(info)
}

/// TESTING


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}