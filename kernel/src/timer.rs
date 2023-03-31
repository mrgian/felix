use crate::pic::PICS;
use core::arch::asm;

pub const TIMER_INT: u8 = 32;

//timer handler
#[naked]
pub extern "C" fn timer() {
    unsafe {
        asm!("call timer_handler", "iretd", options(noreturn));
    }
}

#[no_mangle]
pub extern "C" fn timer_handler() {
    //print!(".");

    PICS.end_interrupt(TIMER_INT);
}
