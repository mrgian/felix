#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod print;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    wait_key_and_reboot();

    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("Loaded! Welcome to Felix {}", VERSION);

    loop {}
}

//TODO: Fix, it's not working
#[allow(overflowing_literals)]
fn wait_key_and_reboot() {
    println!("Press any key to reboot...");

    unsafe {
        asm!("mov ah, 0", "int 0x16", "jmp {0:x}", in(reg) 0x7c00 as u16);
    }
}
