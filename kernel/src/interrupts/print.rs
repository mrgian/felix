use crate::interrupts::pic::PICS;
use core::arch::asm;

pub const PRINT_INT: u8 = 0x80;

//timer handler
#[naked]
pub extern "C" fn print() {
    unsafe {
        asm!(
            "push eax",
            "push ebx",
            "call print_handler",
            "add esp, 8",
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn print_handler(size: u32, address: u32) {
    unsafe {
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
