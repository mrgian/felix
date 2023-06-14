//HELLO
//Simple program to test 

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libfelix;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    libfelix::println!("Ciaoooo {}", 69);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
