#![no_std]
#![no_main]

//use core::arch::asm;
use core::panic::PanicInfo;

#[macro_use]
mod print;

mod fat;
use fat::FatDriver;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("Bootloader loaded!");

    let driver = FatDriver::new();
    driver.load_header();
    driver.load_entries();

    driver.list_entries();


    loop {}
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    println!("Failed loading root!");

    loop {}
}
