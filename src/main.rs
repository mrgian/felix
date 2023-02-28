#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::arch::asm;

global_asm!(include_str!("bootloader/boot.asm"));

//const MSG: &[u8] = b"Hello from Rust!\r\n\0";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*#[no_mangle]
pub extern "C" fn print(message_pointer: *const u8) {
    unsafe{
        asm!(include_str!("bootloader/print.asm"), in(reg) message_pointer);
    }
}*/

fn print(message: &str) {
    unsafe{
        asm!(include_str!("bootloader/print.asm"), in(reg) message.as_ptr());
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let hello = "Hello world from Rust!";
    print(hello);
}

/*#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}*/
