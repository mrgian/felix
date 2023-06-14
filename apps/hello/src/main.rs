//HELLO
//Simple program to test libfelix

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libfelix;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    let a = 0xFFFF;
    libfelix::println!("Hello world! {:X}", a);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
