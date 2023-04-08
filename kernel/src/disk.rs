use core::arch::asm;

//controller registers ports
const DATA_REGISTER: u16 = 0x1f0;
const SECTOR_COUNT_REGISTER: u16 = 0x1f2;
const LBA_LOW_REGISTER: u16 = 0x1f3;
const LBA_MID_REGISTER: u16 = 0x1f4;
const LBA_HIGH_REGISTER: u16 = 0x1f5;
const DRIVE_REGISTER: u16 = 0x1f6;

//port used for both sending command and getting status
const STATUS_COMMAND_REGISTER: u16 = 0x1f7;

//read write command codes
const READ_COMMAND: u8 = 0x20;
const WRITE_COMMAND: u8 = 0x30;

//status register bits
const STATUS_BSY: u8 = 0b10000000;
const STATUS_RDY: u8 = 0b01000000;
const STATUS_DFE: u8 = 0b00100000;
const STATUS_DRQ: u8 = 0b00001000;
const STATUS_ERR: u8 = 0b00000001;

pub static mut DISK: Disk = Disk {};

pub struct Disk {}

impl Disk {
    //read multiple sectors from lba to specified adddress
    pub fn read(&self, target: u32, lba: u32, sectors: u8) {
        //wait until not busy
        while self.is_busy() {}

        unsafe {
            //setup registers
            asm!("out dx, al", in("dx") SECTOR_COUNT_REGISTER, in("al") sectors); //number of setcors to read
            asm!("out dx, al", in("dx") LBA_LOW_REGISTER, in("al") lba as u8); //low 8 bits of lba
            asm!("out dx, al", in("dx") LBA_MID_REGISTER, in("al") (lba >> 8) as u8); //next 8 bits of lba
            asm!("out dx, al", in("dx") LBA_HIGH_REGISTER, in("al") (lba >> 16) as u8); //next 8 bits of lba
            asm!("out dx, al", in("dx") DRIVE_REGISTER, in("al") (0xE0 | ((lba >> 24) & 0xF)) as u8); //0xe0 (master drive) ORed with highest 4 bits of lba

            //disable interrupts before sending command (i still don't what int number the controller generates)
            asm!("cli");

            //send read command to port
            asm!("out dx, al", in("dx") STATUS_COMMAND_REGISTER, in("al") READ_COMMAND);

            //enable interrupts again
            asm!("cli");
        }

        //wait until not busy
        while self.is_busy() {}

        //wait until ready
        while !self.is_ready() {}

        let mut sectors_left = sectors;
        let mut target_address = target;
        while sectors_left > 0 {
            //a sector is 512 byte, buffer size is 4 byte, so loop for 512/4
            for i in 0..128 {
                let buffer: u32;
                unsafe {
                    //read 16 bit from controller buffer 
                    asm!("in eax, dx", out("eax") buffer, in("dx") DATA_REGISTER);

                    //copy buffer in memory pointed by target
                    asm!("mov [{0}], {1}", in(reg) target_address, in(reg) buffer);
                }

                target_address += 4;
                print!("{:X} ", buffer);
            }
            sectors_left -= 1;
        }
    }

    //check if disk is busy
    pub fn is_busy(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }

        //if bsy bit is not 0 return true
        (status & STATUS_BSY) != 0
    }

    //check if disk is ready
    pub fn is_ready(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }

        //if rdy bit is not 0 return true
        (status & STATUS_RDY) != 0
    }
}