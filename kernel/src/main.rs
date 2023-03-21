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

/*pub fn print(message: &str) {
    unsafe {
        asm!("mov si, {0:x}", //move given string address to si
            "2:",
            "lodsb", //load a byte (next character) from si to al
            "or al, al", //bitwise or on al, if al is null set zf to true
            "jz 1f", //if zf is true (end of string) jump to end

            "mov ah, 0x0e",
            "mov bh, 0",
            "int 0x10", //tell the bios to write content of al to screen

            "jmp 2b", //start again
            "1:",
            in(reg) message.as_ptr());
    }
}*/
