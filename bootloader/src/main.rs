#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod print;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

const HEADS_PER_CYLINDER: u16 = 2;
const SECTORS_PER_TRACK: u16 = 18;

#[derive(Debug)]
pub struct CHS {
    cylinder: u16, //cylinder
    head: u16,     //head
    sector: u16,   //sector
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    wait_key_and_reboot();

    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("Bootloader loaded!");

    /*let mut sp: u16;
    unsafe {
        asm!(
            "mov {0:x}, sp",
            out(reg) sp
        );
    }

    println!("Current stack pointer: {:X}", sp);*/

    println!("{:?}", lba_to_chs(2879));

    let a: u16 = 0x01 as u16;
    let b: u16 = 0x00 as u16;
    let s: u16 = a << 8;

    println!("a: {:X}  b: {:X}  s: {:X}", a, b, s);

    loop {}
}

//TODO: Fix, it's not working
#[allow(overflowing_literals)]
fn wait_key_and_reboot() {
    println!("Press any key to reboot...");

    unsafe {
        asm!("mov ah, 0", "int 0x16", "jmp {0:x}", in(reg) 0x7c00 as u16);
    }
}

fn lba_to_chs(lba: u16) -> CHS {
    let cylinder = lba / (HEADS_PER_CYLINDER * SECTORS_PER_TRACK);
    let temp = lba % (HEADS_PER_CYLINDER * SECTORS_PER_TRACK);
    let head = temp / SECTORS_PER_TRACK;
    let sector = temp % SECTORS_PER_TRACK + 1;

    CHS {
        cylinder: cylinder,
        head: head,
        sector: sector,
    }
}
