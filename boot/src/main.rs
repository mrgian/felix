#![no_std]
#![no_main]

mod disk;

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;
use disk::DiskReader;

const BOOTLOADER_LBA: u64 = 2048; //bootloader location logical block address
const BOOTLOADER_SIZE: u16 = 32; //bootloader size in sectors

//set data segments to zero and setup stack
global_asm!(include_str!("boot.asm"));

extern "C" {
    static _bootloader_start: u16;
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    clear();

    print("[!] Starting Felix...\r\n\0");
    print("[!] Loading bootloader...\r\n\0");

    //get bootloader address from linker
    let bootloader_start: *const u16 = unsafe { &_bootloader_start };

    //init disk read
    let target = bootloader_start as u16;
    let mut disk = DiskReader::new(BOOTLOADER_LBA, target);

    //read bootloader to target
    disk.read_sectors(BOOTLOADER_SIZE);

    //jump to first bootloader instruction
    jump(bootloader_start);

    //loop in case bootloader returns
    loop {}
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

//jump execution to given address
fn jump(address: *const u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address as u16);
    }
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    print("[!] Failed loading bootloader!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
