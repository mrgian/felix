#![no_std]
#![no_main]
#![feature(naked_functions)]

#[macro_use]
mod print;

mod exceptions;
mod idt;
mod keyboard;
mod pic;
mod shell;
mod timer;

use core::arch::asm;
use core::panic::PanicInfo;
use idt::IDT;
use pic::PICS;
use shell::SHELL;

//1MiB. TODO: Get those from linker
const KERNEL_START: u32 = 0x0010_0000;
const KERNEL_SIZE: u32 = 0x0010_0000;
const STACK_SIZE: u32 = 0x0010_0000;

const STACK_START: u32 = KERNEL_START + KERNEL_SIZE + STACK_SIZE;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    //setup stack
    unsafe {
        asm!("mov esp, {}", in(reg) STACK_START);
    }

    println!("Welcome to Felix {}!", VERSION);

    unsafe {
        //init interrupt descriptor table
        IDT.init();

        //add CPU exceptions to idt
        IDT.add_exceptions();

        //add hardware interrupts to idt
        IDT.add(timer::TIMER_INT as usize, timer::timer as u32);
        IDT.add(keyboard::KEYBOARD_INT as usize, keyboard::keyboard as u32);

        //load idt
        IDT.load();
    }

    //init programmable interrupt controllers
    PICS.init();

    //enable hardware interrupts
    unsafe {
        asm!("sti");
    }

    unsafe {
        SHELL.init();
    }

    /*unsafe {
        asm!("ud2");
    }*/

    //halt cpu while waiting for interrupts
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC! Info: {}", info);

    loop {}
}
