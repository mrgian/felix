#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    unsafe {
        let a = "ciao\n";

        for i in 0..10 {
            asm!("push eax", "push ebx", "int 0x80", "pop ebx", "pop eax", in("eax") a.as_ptr() as u32, in("ebx") a.len() as u32);
        }
    }

    /*unsafe {
        let dot = "...";
        let mut a: u32 = 0;

        loop {
            while a != 1_048_576 {
                a += 1;
            }
            asm!("mov esi, {0}","int 0x80", in(reg) dot.as_ptr() as u32, in("eax") dot.len() as u32);
            a = 0;
        }
    }*/
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
