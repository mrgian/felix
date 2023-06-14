//SYSTEM CALLS HANDLER

use crate::drivers::pic::PICS;
use crate::syscalls::print;
use core::arch::asm;
use crate::task::TASK_MANAGER;

use core::slice;
use core::str;

pub const SYSCALL_INT: u8 = 0x80;

//SYSCALL IRQ, calls local function using cdecl calling convention
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

//handle syscalls, get syscall number from eax register
#[no_mangle]
pub extern "C" fn syscall_handler(ecx: u32, ebx: u32, eax: u32) {
    unsafe {
        match eax {
            //SYSCALL 0, print string pointed by ebx with lenght specified in ecx
            0 => {
                let s = {
                    let slice = slice::from_raw_parts(ebx as *const u8, ecx as usize);
                    str::from_utf8(slice)
                };

                print::PRINTER.prints(s.unwrap());
            }

            //SYSCALL 1, remove current active task
            1 => {
                TASK_MANAGER.remove_current_task();
            }

            _ => {}
        }

        PICS.end_interrupt(SYSCALL_INT);
    }
}
