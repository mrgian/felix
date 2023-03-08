#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

mod disk;

//set data segments to zero and setup stack
global_asm!(include_str!("boot.asm"));

extern "C" {
    static _kernel_start: u16;
}

#[no_mangle]
pub extern "C" fn main() {
    clear();

    print("Loading Felix...\r\n\0");

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

fn load_kernel(address: *const u16) {
    //TODO: Make a loop that reads one sector at the time instead of telling bios to read multiple sectors,
    //because some bioses have a limit on how many sector you can read

    let lba: u64 = 1; //read from lba 1, 512 bytes after lba 0 (i think)
    let sectors: u16 = 127; //number of sectors to read
    let kernel_offset = address as u16; //offset and
    let kernel_segment = 0x0000 as u16; //segment where to write the read data

    //bios needs disk address packet structure to read from disk
    let disk = disk::DiskAddressPacket::from_lba(lba, sectors, kernel_offset, kernel_segment);

    //actual reading
    //TODO: read more than one sector
    unsafe {
        //disk.load_sectors();
        disk.load_sectors_floppy();
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
    print("Failed loading kernel!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
