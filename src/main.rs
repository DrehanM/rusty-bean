// main.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustybean::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use rustybean::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustybean::memory;
    use rustybean::allocators;
    use x86_64::{structures::paging::Page, VirtAddr};
    use rustybean::memory::BootInfoFrameAllocator;

    rustybean::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocators::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    
    run_actions();

    #[cfg(test)]
    test_main();

    println!("No crash!");

    rustybean::hlt_loop();
}

fn run_actions() {
    println!("Run actions!");
}


// this is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustybean::hlt_loop();
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