#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

#[path="../../disk/disk.rs"]
mod disk;

//set data segments to zero and setup stack
global_asm!(include_str!("boot.asm"));

extern "C" {
    static _bootloader_start: u16;
}

#[no_mangle]
pub extern "C" fn main() {
    clear();

    print("Loading bootloader...\r\n\0");

    //get bootloader address from linker, currently is 0x7e00 (the end of mbr)
    let bootloader_start: *const u16 = unsafe { &_bootloader_start };

    load_bootloader(bootloader_start);
    jump(bootloader_start);
}

//set bios video mode to clear the screen
fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}

//bios interrupt to print to the screen
fn print(message: &str) {
    unsafe {
        asm!("mov si, {0:x}", //move given string address to si
            "2:",
            "lodsb", //load a byte (next character) from si to al
            "or al, al", //bitwise or on al, if al is null set zf to true
            "jz 1f", //if zf is true (end of string) jump to end

            "mov ah, 0x0e",
            "mov bh, 0",
            "int 0x10", //tell the bios to write content of al to screen

            "jmp 2b", //start again
            "1:",
            in(reg) message.as_ptr());
    }
}

fn load_bootloader(address: *const u16) {
    let lba: u16 = 2816; //read from lba 2816 (2880 - 64), last 64 sectors
    let sector_count: u16 = 64; //number of sectors to read
    let buffer = address as u16; //address where to write the data

    let disk = disk::DiskReader::from_lba(lba, sector_count, buffer);

    //actual reading
    disk.load_sectors();
}

//jump execution to given address
fn jump(address: *const u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address as u16);
    }
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    print("Failed loading bootloader!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
