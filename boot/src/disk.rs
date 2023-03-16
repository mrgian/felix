use core::arch::asm;

#[repr(C, packed)]
pub struct DiskAddressPacket {
    size: u8,
    zero: u8,
    sectors: u16,
    offset: u16,
    segment: u16,
    lba: u64,
}

impl DiskAddressPacket {
    pub fn new(lba: u64, buffer: u16) -> Self {
        Self {
            size: 16,
            zero: 0,
            sectors: 32,
            offset: buffer,
            segment: 0x0000,
            lba: lba,
        }
    }

    //read from disk using bios interrupt 0x13
    pub fn read_sector(&self) {
        let dap_address: u16 = self as *const Self as u16;

        unsafe {
            asm!(
                "mov {1:x}, si", // backup si
                "mov si, {0:x}", //put dap address in si
                "int 0x13",
                "jc fail",
                "mov si, {1:x}", // restore si
                in(reg) dap_address,
                out(reg) _,
                in("ax") 0x4200 as u16,
                in("dx") 0x0080 as u16,
            );
        }
    }
}
