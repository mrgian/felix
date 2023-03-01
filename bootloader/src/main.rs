#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

mod dap;

global_asm!(include_str!("boot.asm"));

extern "C" {
    static _kernel_start: u8;
}

#[no_mangle]
pub extern "C" fn main() {
    clean();
    print("Hello!\r\n\0");
    load_kernel();
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

fn load_kernel() {
    print("ld kernl\r\n\0");
    let kernel_start_address: *const u8 = unsafe { &_kernel_start };

    let lba: u64 = 1;
    let sectors: u16 = 1;
    let target = kernel_start_address as u16;


    let dap = dap::DiskAddressPacket::from_lba(
        lba,
        sectors,
        0x7e00 as u16,
        0x0000 as u16,
    );
    unsafe {
        dap.perform_load();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
