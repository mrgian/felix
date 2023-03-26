#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[macro_use]
mod print;

mod idt;
use idt::InterruptDescriptorTable;

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

    let idt = InterruptDescriptorTable::new(handler as u32);

    let addr = &idt as *const InterruptDescriptorTable;

    println!("Load: {:X}", addr as u32);

    let a: [u64; 3001] = [0xffff_ffff; 3001];
    let reff = &a[3000] as *const u64;

    println!("Pos: {:X}, Value: {:X}", reff as u32, a[3000]);

    //crash();

    //println!("Not crashed!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC! Info: {}", info);

    loop {}
}

fn crash() {
    unsafe {
        asm!("div bl", in("al") 0x00 as u8, in("bl") 0x00 as u8);
    }
}

pub extern "C" fn handler() -> ! {
    println!("CPU EXCEPTION!");
    loop {}
}
