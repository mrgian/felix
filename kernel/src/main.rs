#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod print;

mod idt;
use idt::InterruptDescriptorTable;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Felix {}!", VERSION);

    let idt = InterruptDescriptorTable::new(handler as u32);
    idt.load();

    crash();

    println!("Not crashed!");

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
