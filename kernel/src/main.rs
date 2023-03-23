#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod print;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    print!("Welcome to Felix {}!", VERSION);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

