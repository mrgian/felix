use crate::interrupts::pic::PICS;
use core::arch::asm;

pub const PRINT_INT: u8 = 0x80;

//timer handler
#[naked]
pub extern "C" fn print() {
    unsafe {
        asm!("call print_handler", "iretd", options(noreturn));
    }
}

#[no_mangle]
pub extern "C" fn print_handler() {
    let address: u32;
    let size: u32;

    unsafe {
        asm!("mov {0}, esi","mov {1}, eax", out(reg) address, out(reg) size);

        let mut pointer = address as *const u8;
        let mut c = *pointer as char;

        for _i in 0..size {
            print!("{}", c);
            pointer = pointer.byte_add(1);
            c = *pointer as char;
        }
    }

    PICS.end_interrupt(PRINT_INT);
}
