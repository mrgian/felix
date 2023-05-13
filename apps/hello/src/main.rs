#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use stdio;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    unsafe {
        stdio::print!("Ciaoooo {}", 69);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
