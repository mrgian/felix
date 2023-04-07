use core::arch::asm;

const STATUS_REGISTER: u16 = 0x1f7;

const STATUS_BSY: u8 = 0b10000000;
const STATUS_RDY: u8 = 0b01000000;
const STATUS_DFE: u8 = 0b00100000;
const STATUS_DRQ: u8 = 0b00001000;
const STATUS_ERR: u8 = 0b00000001;

pub static mut DISK: Disk = Disk {};

pub struct Disk {}

impl Disk {
    pub fn read(&self, target: &mut [u32], lba: u32, sectors: u8) {
        //loop while busy
        while self.is_busy() {}

        unsafe {
            asm!("out dx, al", in("dx") 0x1f6 as u16, in("al") (0xE0 | ((lba >> 24) & 0xF)) as u8);
            asm!("out dx, al", in("dx") 0x1f2 as u16, in("al") sectors);
            asm!("out dx, al", in("dx") 0x1f3 as u16, in("al") lba as u8);
            asm!("out dx, al", in("dx") 0x1f4 as u16, in("al") (lba >> 8) as u8);
            asm!("out dx, al", in("dx") 0x1f5 as u16, in("al") (lba >> 16) as u8);
            asm!("out dx, al", in("dx") 0x1f7 as u16, in("al") 0x30 as u8);
        }

        while self.is_busy() {}
        while !self.is_ready() {}

        for i in 0..256 {
            let long: u32;
            unsafe {
                asm!("in eax, dx", out("eax") long, in("dx") 0x1f0 as u16);
            }

            println!("{:X}", long);
        }

        /*port_byte_out(0x1F6,0xE0 | ((LBA >>24) & 0xF));
        port_byte_out(0x1F2,sector_count);
        port_byte_out(0x1F3, (uint8_t) LBA);
        port_byte_out(0x1F4, (uint8_t)(LBA >> 8));
        port_byte_out(0x1F5, (uint8_t)(LBA >> 16));
        port_byte_out(0x1F7,0x30);*/
    }

    pub fn is_busy(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_REGISTER as u16);
        }

        //if bsy bit is not 0 return true
        (status & STATUS_BSY) != 0
    }

    pub fn is_ready(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_REGISTER as u16);
        }

        //if rdy bit is not 0 return true
        (status & STATUS_RDY) != 0
    }
}
