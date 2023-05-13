#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(pointer_byte_offsets)]

mod drivers;
mod filesystem;
mod interrupts;
mod shell;
mod syscalls;
mod task;

use core::arch::asm;
use core::panic::PanicInfo;
use drivers::disk::DISK;
use drivers::pic::PICS;
use filesystem::fat::FAT;
use interrupts::idt::IDT;
use shell::SHELL;
use syscalls::print::PRINTER;

use stdio;

//use task::Task;
//use task::TASK_MANAGER;

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

        //add system call handler interrupt
        IDT.add(
            syscalls::handler::SYSCALL_INT as usize,
            syscalls::handler::syscall as u32,
        );

        //add keyboard interrupt to idt
        IDT.add(
            drivers::keyboard::KEYBOARD_INT as usize,
            drivers::keyboard::keyboard as u32,
        );

        //load idt
        IDT.load();
    }

    //init programmable interrupt controllers
    PICS.init();

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

    /*unsafe {
        stdio::PRINTER.printc('X');
    }*/

    //bochs magic breakpoint
    /*unsafe {
        asm!("xchg bx, bx");
    }*/

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
    crate::println!("PANIC! Info: {}", info);

    loop {}
}

fn print_info() {
    unsafe {
        PRINTER.set_colors(0xf, 0);
    }

    crate::println!();
    crate::println!("FELIX {}", VERSION);
    crate::println!("Copyright (c) 2023 Gianmatteo Palmieri");
    crate::println!();

    unsafe {
        PRINTER.reset_colors();
    }
}
