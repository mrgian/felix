#![no_std]
#![no_main]
#![feature(naked_functions)]

#[macro_use]
extern crate lazy_static;

use core::arch::asm;
use core::panic::PanicInfo;

#[macro_use]
mod print;

mod idt;
use idt::InterruptDescriptorTable;

mod pic;
use pic::Pics;

//1MiB. TODO: Get those from linker
const KERNEL_START: u32 = 0x0010_0000;
const KERNEL_SIZE: u32 = 0x0010_0000;
const STACK_SIZE: u32 = 0x0010_0000;

const STACK_START: u32 = KERNEL_START + KERNEL_SIZE + STACK_SIZE;

const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static!{
    static ref PICS: Pics = Pics::new();
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    //setup stack
    unsafe {
        asm!("mov esp, {}", in(reg) STACK_START);
    }

    println!("Welcome to Felix {}!", VERSION);

    //let pics = Pics::new();
    PICS.init();

    let mut idt = InterruptDescriptorTable::new();
    idt.add_exceptions();
    idt.add(32,timer as u32);
    idt.load();

    unsafe {
        asm!("sti");
    }

    println!("Not crashed!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC! Info: {}", info);

    loop {}
}

#[naked]
pub extern "C" fn timer() {
    unsafe {
        asm!(
            "call print_dot",
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn print_dot() {
    print!(".");

    PICS.end_interrupt(32);
}
