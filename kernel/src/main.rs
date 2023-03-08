#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod print;

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
    println!("Loaded! Welcome to Felix!\n\r");

    let mut sp: u16;
    unsafe {
        asm!(
            "mov {0:x}, sp",
            out(reg) sp
        );
    }

    println!("Current stack pointer: {:X}\r\n", sp);

    loop {}
}
