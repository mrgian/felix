#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        let a = "hello from app!";

        asm!("mov esi, {0}","int 0x80", in(reg) a.as_ptr() as u32, in("eax") a.len() as u32);

        loop {}
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
