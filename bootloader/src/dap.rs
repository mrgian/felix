use core::arch::asm;

#[repr(C, packed)]
#[allow(dead_code)]
pub struct DiskAddressPacket {
    packet_size: u8,
    zero: u8,
    number_of_sectors: u16,
    offset: u16,
    segment: u16,
    start_lba: u64,
}

impl DiskAddressPacket {
    pub fn from_lba(
        start_lba: u64,
        number_of_sectors: u16,
        target_offset: u16,
        target_segment: u16,
    ) -> Self {
        Self {
            packet_size: 0x10,
            zero: 0,
            number_of_sectors,
            offset: target_offset,
            segment: target_segment,
            start_lba,
        }
    }

    pub unsafe fn load_sectors(&self) {
        let self_addr = self as *const Self as u16;
        unsafe {
            asm!(
                "push 0x7a",
                "mov {1:x}, si",
                "mov si, {0:x}",
                "int 0x13",
                //"jc fail",
                "pop si",
                "mov si, {1:x}",
                in(reg) self_addr,
                out(reg) _,
                in("ax") 0x4200,
                in("dx") 0x0080,
            );
        }
    }
}
