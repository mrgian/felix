#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!("xchg bx, bx");

        let ptr = 0x0035_0000 as *mut u32;
        *ptr = 0xdead_beef;

        loop {}
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
