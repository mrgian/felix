#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

mod dap;

global_asm!(include_str!("boot.asm"));

extern "C" {
    static _kernel_start: u16;
}

#[no_mangle]
pub extern "C" fn main() {
    clean();

    print("Loading...\r\n\0");

    load_kernel();

    jump_to_kernel();
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
    let kernel_start: *const u16 = unsafe { &_kernel_start };

    let lba: u64 = 1;
    let sectors: u16 = 1;
    let kernel_offset = kernel_start as u16;
    let kernel_segment = 0x0000 as u16;

    let dap = dap::DiskAddressPacket::from_lba(lba, sectors, kernel_offset, kernel_segment);

    unsafe {
        dap.load_sectors();
    }
}

fn jump_to_kernel() {
    unsafe {
        asm!("jmp {}", in(reg) 0x7e00);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
