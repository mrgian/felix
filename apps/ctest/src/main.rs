//HELLO
//Simple program to test libfelix

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libfelix;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    let mut a: u32 = 0;
    let mut b: u8 = 0;
    loop {
        if a == 100_000_000 {
            libfelix::println!("Process C running. {}% complete.", b);
            a = 0;
            b += 1;

            if b == 100 {
                libfelix::println!("Process C complete.");
                break;
            }
        }
        a += 1;
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
