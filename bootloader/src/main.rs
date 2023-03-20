#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[macro_use]
mod print;

mod disk;
use disk::DiskReader;

mod gdt;
use gdt::GlobalDescriptorTable;

//const VERSION: &str = env!("CARGO_PKG_VERSION");
const KERNEL_LBA: u64 = 2048 + 32; //kernel location logical block address (bootloader lba + bootloader size)
const KERNEL_SIZE: u16 = 32; //kernel size in sectors
const KERNEL_TARGET: u16 = 0xbe00; //where to put kernel in memory

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

//bootloader entry point
#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("[!] Loading kernel...");

    let mut disk = DiskReader::new(KERNEL_LBA, KERNEL_TARGET);
    disk.read_sectors(KERNEL_SIZE);

    println!("[!] Loading Global Descriptor Table...");
    let gdt = GlobalDescriptorTable::new();

    gdt.load();

    println!("[!] Switching to 32bit protected mode and jumping to kernel...");

    unsafe {
        //enable protected mode in cr0 register
        asm!("mov eax, cr0", "or al, 1", "mov cr0, eax");

        //push kernel address
        asm!(
            "push {0:e}",
            in(reg) KERNEL_TARGET as u32,
        );

        //jump to protected mode
        asm!("ljmp $0x8, $2f", "2:", options(att_syntax));

        //protected mode start
        asm!(
            ".code32",

            //setup segment registers
            "mov {0}, 0x10",
            "mov ds, {0}",
            "mov es, {0}",
            "mov ss, {0}",

            //jump to kernel
            "pop {1}",
            "call {1:e}",

            out(reg) _,
            in(reg) KERNEL_TARGET as u32,
        );
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    println!("[!] Read fail!");

    loop {}
}
