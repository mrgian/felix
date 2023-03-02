#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

//TODO: Handle kernel panic, instead of looping
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    //loops but it shouldn't
    //like people when they have a panic attack
    //their mind loops instead of dealing with problems
    loop {}
}

//bios interrupt to print to the screen
//TODO: Implement a printf function that is able to format strings
fn print(message: &str) {
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
}

//set bios video mode to clear the screen
#[allow(dead_code)]
fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    print("Loaded!\r\nWelcome to Felix!");

    loop {}
}
