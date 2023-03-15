use core::arch::asm;

const HEADS_PER_CYLINDER: u16 = 2;
const SECTORS_PER_TRACK: u16 = 18;

pub struct DiskReader {
    sector_count: u16, //how many sector to read
    buffer: u16,       //where to write data
    chs: CHS,          //cylinder-head-sector
    drive_number: u16, //drive number (0x00 for floppy)
}

pub struct CHS {
    cylinder: u16, //cylinder
    head: u16,     //head
    sector: u16,   //sector
}

impl DiskReader {
    //create a diskreader using a logic block address
    pub fn from_lba(lba: u16, sector_count: u16, buffer: u16) -> Self {
        Self {
            sector_count: sector_count,
            buffer: buffer,
            chs: Self::lba_to_chs(lba),
            drive_number: 0,
        }
    }

    //actual reading using bios interrupt 0x13
    pub fn load_sectors(&self) {
        //Link to bible: https://wiki.osdev.org/Disk_access_using_the_BIOS_(INT_13h)

        //Before calling bios interrupt register should be like this:
        //
        //ah = 2
        //al = sector_count
        //dh = head
        //dl = drive number
        //bx = buffer

        //cx         = [  CH  ] [  CL  ]
        //cylinder   = XXXXXXXX XX
        //sector     =            XXXXXX

        unsafe {
            asm!(
                "int 0x13",
                "jc fail",
                in("ah") 0x02 as u8,
                in("al") self.sector_count as u8,
                in("ch") (self.chs.cylinder as u8 & 0xff) as u8,
                in("cl") (self.chs.sector | ((self.chs.cylinder >> 2) & 0xc0)) as u8,

                //in("dh") self.chs.head as u8,
                //in("dl") self.drive_number as u8,

                //rust doesn't let me set dh and dl, so i set dx with this trick
                in("dx") ((self.chs.head << 8) + self.drive_number) as u16,

                in("bx") self.buffer as u16,
            );
        }
    }

    //alogorithm to convert from logic-block-address to cylinder-head-sector
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
}
