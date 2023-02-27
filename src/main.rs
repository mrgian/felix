#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

const x: &str = "Hello world";
global_asm!(include_str!("bootloader/boot.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}*/
