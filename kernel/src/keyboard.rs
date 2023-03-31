use crate::pic::PICS;
use core::arch::asm;

pub const KEYBOARD_INT: u8 = 33;

//keyboard handler
#[naked]
pub extern "C" fn keyboard() {
    unsafe {
        asm!("call keyboard_handler", "iretd", options(noreturn));
    }
}

#[no_mangle]
pub extern "C" fn keyboard_handler() {
    //read scancode from keyboard controller
    let scancode: u8;
    unsafe {
        asm!("in al, dx", out("al") scancode, in("dx") 0x60 as u16);
    }

    println!("KEYBOARD INTERRUPT! Scancode: {:X} ", scancode);

    PICS.end_interrupt(KEYBOARD_INT);
}
