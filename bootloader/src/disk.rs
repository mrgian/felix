use core::arch::asm;
use core::mem;
use core::ptr;

const SECTOR_SIZE: u16 = 512; //sector size in bytes

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
    target: u16,
}

impl DiskReader {
    pub fn new(lba: u64, target: u16) -> Self {
        Self {
            lba: lba,
            target: target,
        }
    }

    //read one sector from disk
    pub fn read_sector(&self) {
        //init dap
        let dap = DiskAddressPacket {
            size: mem::size_of::<DiskAddressPacket>() as u8,
            zero: 0,
            sectors: 1,
            offset: self.target,
            segment: 0x0000,
            lba: self.lba,
        };

        //get dap addrees
        let dap_address = ptr::addr_of!(dap) as u16;

        //bios int 0x13
        unsafe {
            asm!(
                "mov {1:x}, si", //backup si
                "mov si, {0:x}", //put dap address in si
                "int 0x13",
                "jc fail",
                "mov si, {1:x}", //restore si
                in(reg) dap_address,
                out(reg) _,
                in("ax") 0x4200 as u16,
                in("dx") 0x0080 as u16,
            );
        }
    }

    //read multiple sectors
    pub fn read_sectors(&mut self, sectors: u16) {
        let mut sectors_left = sectors;

        //read one sector at a time and stop when there are no more sectors to read left
        while sectors_left > 0 {
            self.read_sector();

            self.target += SECTOR_SIZE;
            self.lba += 1;
            sectors_left -= 1;
        }
    }
}
