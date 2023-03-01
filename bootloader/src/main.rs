#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

mod dap;

//set data segments to zero and setup stack
global_asm!(include_str!("boot.asm"));

extern "C" {
    static _kernel_start: u16;
}

#[no_mangle]
pub extern "C" fn main() {
    clear();

    //uncommenting this makes the executable bigger than the 512 byte limit in debug mode
    //TODO: optimize to make it fit or print only in release mode
    //print("Loading...\r\n\0");

    //get kernel address from linker, currently is 0x7e00 (the end of mbr)
    let kernel_start: *const u16 = unsafe { &_kernel_start };

    load_kernel(kernel_start);
    jump(kernel_start);
}

//set bios video mode to clear the screen
fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}

//bios interrupt to print to the screen
#[allow(dead_code)]
fn print(message: &str) {
    unsafe {
        asm!(include_str!("print.asm"), in(reg) message.as_ptr());
    }
}

fn load_kernel(address: *const u16) {
    let lba: u64 = 1; //read from lba 1, 512 bytes after lba 0 (i think)
    let sectors: u16 = 1; //read only one sector
    let kernel_offset = address as u16; //offset and
    let kernel_segment = 0x0000 as u16; //segment where to write the read data

    //bios needs disk address packet structure to read from disk
    let dap = dap::DiskAddressPacket::from_lba(lba, sectors, kernel_offset, kernel_segment);

    //actual reading
    //TODO: read more than one sector
    unsafe {
        dap.load_sectors();
    }
}

//jump execution to given address
fn jump(address: *const u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address as u16);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
