#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::arch::asm;

global_asm!(include_str!("bootloader/boot.asm"));

const msg: &str = "Hello from rust porco diooooo\r\n";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn print() {
    unsafe{
        asm!(include_str!("bootloader/print.asm"), in(reg) msg.as_ptr());
    }
}

/*#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}*/
