#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    unsafe {
        let a = "ciao\n";

        for c in a.bytes() {
            asm!("push eax", "push ebx", "int 0x80", "pop ebx", "pop eax", in("eax") 0, in("ebx") c as u32);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
