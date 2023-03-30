#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::asm;
use core::panic::PanicInfo;

#[macro_use]
mod print;

mod idt;
use idt::InterruptDescriptorTable;

mod pic;
use pic::PICS;

//1MiB. TODO: Get those from linker
const KERNEL_START: u32 = 0x0010_0000;
const KERNEL_SIZE: u32 = 0x0010_0000;
const STACK_SIZE: u32 = 0x0010_0000;

const STACK_START: u32 = KERNEL_START + KERNEL_SIZE + STACK_SIZE;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const TIMER_INT: u8 = 32;
const KEYBOARD_INT: u8 = 33;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    //setup stack
    unsafe {
        asm!("mov esp, {}", in(reg) STACK_START);
    }

    println!("Welcome to Felix {}!", VERSION);

    //init programmable interrupt controllers
    PICS.init();

    //init interrupt descriptor table
    let mut idt = InterruptDescriptorTable::new();

    //add CPU exceptions to idt
    idt.add_exceptions();

    //add hardware interrupts to idt
    idt.add(TIMER_INT as usize, timer as u32);
    idt.add(KEYBOARD_INT as usize, keyboard as u32);

    //load idt
    idt.load();

    //enable hardware interrupts
    unsafe {
        asm!("sti");
    }

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
