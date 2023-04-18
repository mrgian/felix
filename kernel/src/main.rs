#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(pointer_byte_offsets)]

#[macro_use]
mod print;

mod interrupts;
mod keyboard;
mod shell;

mod disk;
mod fat;

mod paging;

use core::arch::asm;
use core::panic::PanicInfo;
use disk::DISK;
use fat::FAT;
use interrupts::idt::IDT;
use interrupts::pic::PICS;
use paging::PAGE_DIRECTORY;
use print::PRINTER;
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

    unsafe {
        //init idt
        IDT.init();

        //add CPU exceptions to idt
        IDT.add_exceptions();

        //add timer interrupt to idt
        IDT.add(
            interrupts::timer::TIMER_INT as usize,
            interrupts::timer::timer as u32,
        );

        //add print system call
        IDT.add(
            interrupts::print::PRINT_INT as usize,
            interrupts::print::print as u32,
        );

        //add keyboard interrupt to idt
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
        //check if ata drive is working
        DISK.check();
    }

    print_info();

    unsafe {
        if DISK.enabled {
            //init filesystem
            FAT.load_header();
            FAT.load_entries();
            FAT.load_table();
        }
    }

    unsafe {
        //init felix shell
        SHELL.init();
    }

    unsafe {
        PAGE_DIRECTORY.init();

        let a = (&PAGE_DIRECTORY as *const paging::PageDirectory) as u32;
        asm!("xchg bx, bx");

        asm!("mov cr3, eax","mov eax, cr0","or eax, 0x80000001","mov cr0, eax", in("eax") a);
        asm!("xchg bx, bx");
    }

    unsafe {
        let a = "testtesttest";

        asm!("mov esi, {0}","int 0x80", in(reg) a.as_ptr() as u32, in("eax") a.len() as u32);
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

fn print_info() {
    unsafe {
        PRINTER.set_colors(0xf, 0);
    }

    println!();
    println!("FELIX {}", VERSION);
    println!("Copyright (c) 2023 Gianmatteo Palmieri");
    println!();

    unsafe {
        PRINTER.reset_colors();
    }
}
