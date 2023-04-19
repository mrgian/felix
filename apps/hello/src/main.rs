#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}