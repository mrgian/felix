#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("bootloader/boot.asm"));
global_asm!("message: .string \"Hello world from Rust!\r\n\"");

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}*/
