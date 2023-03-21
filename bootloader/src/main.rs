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
const KERNEL_LBA: u64 = 4096; //kernel location logical block address
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
    println!("[!] Switching to unreal mode...");

    unreal_mode();

    println!("[!] Loading kernel...");

    let mut disk = DiskReader::new(KERNEL_LBA, KERNEL_TARGET);
    disk.read_sectors(KERNEL_SIZE);

    println!("[!] Loading Global Descriptor Table...");
    let gdt = GlobalDescriptorTable::new();

    gdt.load();

    println!("[!] Switching to 32bit protected mode and jumping to kernel...");

    protected_mode();

    loop {}
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    println!("[!] Read fail!");

    loop {}
}

//switch to 32bit protected mode
fn protected_mode() {
    unsafe {
        //enable protected mode in cr0 register
        asm!("mov eax, cr0", "or al, 1", "mov cr0, eax");

        //push kernel address
        asm!(
            "push {}",
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
            "call {1}",

            out(reg) _,
            in(reg) KERNEL_TARGET as u32,
        );
    }
}

//switch to 16bit unreal mode, this mode allows to use 32bit registers in 16bit mode
//this mode is needed to copy from buffer to protected mode memory
fn unreal_mode() {  
    //backup segment registers
    let ds: u16;
    let ss: u16;
    unsafe {
        asm!("mov {0:x}, ds", out(reg) ds, options(nomem, nostack, preserves_flags));
        asm!("mov {0:x}, ss", out(reg) ss, options(nomem, nostack, preserves_flags));
    }

    //load gdt
    let gdt = GlobalDescriptorTable::new();
    gdt.load();

    unsafe {
        //backup cr0 register
        let mut cr0: u32;
        asm!("mov {:e}, cr0", out(reg) cr0);

        //set cr0 protected bit
        let cr0_protected = cr0 | 1;
        asm!("mov cr0, {:e}", in(reg) cr0_protected);

        //setup segment registers
        asm!("mov {0}, 0x10", "mov ds, {0}", "mov ss, {0}", out(reg) _);

        //restore cr0 register
        asm!("mov cr0, {:e}", in(reg) cr0);

        //restore segment registers
        asm!("mov ds, {0:x}", in(reg) ds);
        asm!("mov ss, {0:x}", in(reg) ss);

        //set inerrupt flag
        //asm!("sti");
    }

    unsafe {
        asm!("mov [{}], {}", in(reg) 0x0010_0000, in(reg_byte) 0xde as u8);
    }
}

