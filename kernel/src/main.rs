#![no_std]
#![no_main]

//use core::arch::asm;
use core::panic::PanicInfo;

mod print;
use print::printc;
use print::prints;

//TODO: Handle kernel panic, instead of looping
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    //loops but it shouldn't
    //like people when they have a panic attack
    //their mind loops instead of dealing with problems
    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    prints("Loaded!\r\nWelcome to Felix!\r\n\0");
    printc('X');
    printc('o');

    loop {}
}
