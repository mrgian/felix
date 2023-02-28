#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    let hello = "Hello world from Rust!";
    clean();
    print(hello);
}

fn clean() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}

fn print(message: &str) {
    unsafe {
        asm!(include_str!("print.asm"), in(reg) message.as_ptr());
    }
}
