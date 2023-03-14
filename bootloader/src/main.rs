#![no_std]
#![no_main]

//use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr;

mod print;

/*#[path = "../../disk/disk.rs"]
mod disk;*/

#[path = "../../disk/fat.rs"]
mod fat;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("Bootloader loaded!");

    let root: [fat::Entry; 224] = [fat::Entry::default(); 224];
    let root_address = ptr::addr_of!(root) as u16;

    fat::load_root(root_address);

    println!("Loaded root at: {:X}", root_address);

    list_entries(&root);


    loop {}
}

pub fn list_entries(entries: &[fat::Entry]) {
    println!("Listing root directory:");

    println!();

    for e in 0..224 {
        let name = entries[e].name;
        if name[0] != 0 {
            for c in 0..11 {
                print!("{}", name[c] as char);
            }
            println!();
        }
    }
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    println!("Failed loading root!");

    loop {}
}
