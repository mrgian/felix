#![no_std]
#![no_main]

//use core::arch::asm;
use core::fmt::Write;
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
    //TODO: make a println macro
    let mut printer = print::Printer {};
    write!(printer, "Loaded!\n\rWelcome to Felix\n\r").unwrap();
    write!(printer, "Hello {}\n\r", "world").unwrap();
    write!(printer, "{} {}\n\r", 69, 420).unwrap();

    /*let mut x: u16;
    unsafe {
        asm!(
            "mov {x}, ebx",
            x = out(reg) x
        );
    }*/

    //write!(printer, "Current ebx reg: {:X}", x).unwrap();

    loop {}
}
