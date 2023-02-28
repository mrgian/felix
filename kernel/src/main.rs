#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn print(message: &str) {
    unsafe{
        asm!(include_str!("print.asm"), in(reg) message.as_ptr());
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = "Hello world from Rust Kernel!";
    print(hello);

    loop{}
}
