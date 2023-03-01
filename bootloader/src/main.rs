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
    clear();

    //don't make this string too big, you might exceed the 512 byte limit
    print("Loading...\r\n\0");

    let kernel_start: *const u16 = unsafe { &_kernel_start };

    load_kernel(kernel_start);
    jump(kernel_start);
}

//sets bios video mode to clear the screen
fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}

//uses bios interrupt to print to the screen
//check print.asm for more info
fn print(message: &str) {
    unsafe {
        asm!(include_str!("print.asm"), in(reg) message.as_ptr());
    }
}

fn load_kernel(address: *const u16) {
    let lba: u64 = 1;
    let sectors: u16 = 1;
    let kernel_offset = address as u16;
    let kernel_segment = 0x0000 as u16;

    let dap = dap::DiskAddressPacket::from_lba(lba, sectors, kernel_offset, kernel_segment);

    unsafe {
        dap.load_sectors();
    }
}

fn jump(address: *const u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address as u16);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
