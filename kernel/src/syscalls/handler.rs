//SYSTEM CALLS HANDLER

use crate::drivers::pic::PICS;
use crate::syscalls::print;
use core::arch::asm;

use core::slice;
use core::str;

pub const SYSCALL_INT: u8 = 0x80;

//timer handler
#[naked]
pub extern "C" fn syscall() {
    unsafe {
        asm!(
            "push eax",
            "push ebx",
            "push ecx",
            "call syscall_handler",
            "add esp, 12",
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn syscall_handler(ecx: u32, ebx: u32, eax: u32) {
    unsafe {
        match eax {
            0 => {
                let s = unsafe {
                    let slice = slice::from_raw_parts(ebx as *const u8, ecx as usize);
                    str::from_utf8(slice)
                };

                print::PRINTER.prints(s.unwrap());
            }

            _ => {}
        }

        PICS.end_interrupt(SYSCALL_INT);
    }
}
