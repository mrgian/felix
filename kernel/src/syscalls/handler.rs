//SYSTEM CALLS HANDLER

use crate::drivers::pic::PICS;
use crate::syscalls::print;
use core::arch::asm;

pub const SYSCALL_INT: u8 = 0x80;

//timer handler
#[naked]
pub extern "C" fn syscall() {
    unsafe {
        asm!(
            "push eax",
            "push ebx",
            "call syscall_handler",
            "add esp, 8",
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn syscall_handler(data: u32, id: u32) {
    unsafe {
        match id {
            0 => {
                let c = (data as u8) as char;
                print::PRINTER.printc(c);
            }

            _ => {}
        }

        PICS.end_interrupt(SYSCALL_INT);
    }
}
