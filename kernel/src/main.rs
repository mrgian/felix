#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    print("Hello from kernel\0");

    loop {}
}

fn print(message: &str) {
    unsafe {
        asm!("mov esi, {0:e}",
            "mov edi, 0x000b8000",
            "cld",
            "2:",
            "lodsb",
            "or al, al",
            "jz 1f",

            "mov [edi], al",
            "inc edi",
            "inc edi",

            "jmp 2b",
            "1:",
            in(reg) message.as_ptr());
    }
}
