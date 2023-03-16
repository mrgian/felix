#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

mod disk;
use disk::DiskAddressPacket;

//set data segments to zero and setup stack
global_asm!(include_str!("boot.asm"));

extern "C" {
    static _bootloader_start: u16;
}

#[no_mangle]
pub extern "C" fn main() {
    clear();

    print("Felix 0.1\r\n\0");
    print("Loading bootloader...\r\n\0");

    //get bootloader address from linker, currently is 0x7e00 (the end of mbr)
    let bootloader_start: *const u16 = unsafe { &_bootloader_start };

    let lba = 2048;
    let mut count = 2;
    let mut address = bootloader_start as u16;


    while count > 0 {
        let disk = DiskAddressPacket::new(lba, address);

        print("Loading 32 sectors\r\n\0");
        disk.read_sector();
        print("Loaded.\r\n\0");

        address += 512 * 32;
        count -= 1;
    }

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
