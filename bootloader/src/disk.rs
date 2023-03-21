use core::arch::asm;
use core::mem;

const SECTOR_SIZE: u64 = 512;

#[repr(C, packed)]
struct DiskAddressPacket {
    size: u8,     //size of dap
    zero: u8,     //always zero
    sectors: u16, //sectors to read
    offset: u16,  //target offset
    segment: u16, //target segment
    lba: u64,     //logical block address
}

pub struct DiskReader {
    lba: u64,
    buffer: u16,
}

impl DiskReader {
    pub fn new(lba: u64, buffer: u16) -> Self {
        Self {
            lba: lba,
            buffer: buffer,
        }
    }

    //read one sector from disk
    pub fn read_sector(&self) {
        //init dap
        let dap = DiskAddressPacket {
            size: mem::size_of::<DiskAddressPacket>() as u8,
            zero: 0,
            sectors: 1,
            offset: self.buffer,
            segment: 0x0000,
            lba: self.lba,
        };

        //get dap address
        let dap_address = &dap as *const DiskAddressPacket;

        //bios int 0x13
        unsafe {
            asm!(
                "mov {1:x}, si", //backup si
                "mov si, {0:x}", //put dap address in si
                "int 0x13",
                "jc fail",
                "mov si, {1:x}", //restore si
                in(reg) dap_address as u16,
                out(reg) _,
                in("ax") 0x4200 as u16,
                in("dx") 0x0080 as u16,
            );
        }
    }

    //read multiple sectors, and copy sectors to specified target
    pub fn read_sectors(&mut self, sectors: u16, target: u32) {
        let mut sectors_left = sectors;
        let mut current_target = target;

        //read one sector at a time and stop when there are no more sectors to read left
        while sectors_left > 0 {
            self.read_sector();

            let mut byte_address = self.buffer;

            //for each sector copy byte by byte from buffer to target
            for _byte_index in 0..SECTOR_SIZE {
                unsafe {
                    let mut byte: u8;

                    asm!("mov {0}, [{1:e}]", out(reg_byte) byte, in(reg) byte_address);

                    asm!("mov [{0:e}], {1}", in(reg) current_target, in(reg_byte) byte);
                }

                //increment target and byte address by one byte
                current_target += 1;
                byte_address += 1;
            }

            self.lba += 1;
            sectors_left -= 1;

            Self::print_status(sectors, sectors_left);
        }

        println!();
    }

    //print a period every 1024 sectors read
    fn print_status(sectors: u16, sectors_left: u16) {
        let sectors_read = sectors - sectors_left;
        if (sectors_read % 1024) == 0 {
            print!(".");
        }
    }
}
