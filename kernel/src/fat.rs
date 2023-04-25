use crate::drivers::disk::DISK;
use core::mem;

//FAT DRIVER
//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut FAT: FatDriver = FatDriver {
    header: NULL_HEADER,
    entries: [NULL_ENTRY; ENTRY_COUNT],
    table: [0; FAT_SIZE],
    buffer: [0; 2048],
};

const ENTRY_COUNT: usize = 512;
const FAT_START: u16 = 36864;

const FAT_SIZE: usize = 256;

//FAT16 header
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Header {
    boot_jump_instructions: [u8; 3],

    //bios parameter block
    oem_identifier: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    fat_count: u8,
    dir_entries_count: u16,
    total_sectors: u16,
    media_descriptor_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    heads: u16,
    hidden_sectors: u32,
    large_sector_count: u32,

    //extended boot record
    drive_number: u8,
    reserved: u8,
    signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    system_id: [u8; 8],
    zero: [u8; 460], //needed to make struct 512 bytes big
}

static NULL_HEADER: Header = Header {
    boot_jump_instructions: [0; 3],

    oem_identifier: [0; 8],
    bytes_per_sector: 0,
    sectors_per_cluster: 0,
    reserved_sectors: 0,
    fat_count: 0,
    dir_entries_count: 0,
    total_sectors: 0,
    media_descriptor_type: 0,
    sectors_per_fat: 0,
    sectors_per_track: 0,
    heads: 0,
    hidden_sectors: 0,
    large_sector_count: 0,

    drive_number: 0,
    reserved: 0,
    signature: 0,
    volume_id: 0,
    volume_label: [0; 11],
    system_id: [0; 8],
    zero: [0; 460],
};

//FAT file entry struct
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Entry {
    pub name: [u8; 11],
    attributes: u8,
    reserved: u8,
    created_time_tenths: u8,
    created_time: u16,
    created_date: u16,
    accessed_date: u16,
    first_cluster_high: u16,
    modified_time: u16,
    modified_date: u16,
    first_cluster_low: u16,
    size: u32,
}

static NULL_ENTRY: Entry = Entry {
    name: [0; 11],
    attributes: 0,
    reserved: 0,
    created_time_tenths: 0,
    created_time: 0,
    created_date: 0,
    accessed_date: 0,
    first_cluster_high: 0,
    modified_time: 0,
    modified_date: 0,
    first_cluster_low: 0,
    size: 0,
};

pub struct FatDriver {
    pub header: Header,
    entries: [Entry; ENTRY_COUNT], //the root directory is an array of file entries
    table: [u16; FAT_SIZE],
    pub buffer: [u8; 2048],
}

impl FatDriver {
    //get header address and overwrite that mem location with data from boot sector
    pub fn load_header(&mut self) {
        let target = &mut self.header as *mut Header;

        let lba: u64 = FAT_START as u64;
        let sectors: u16 = 1;

        unsafe {
            DISK.read(target, lba, sectors);
        }
    }

    //get entries array address and overwrite that mem location with data from root directory
    //calculate size and position of root direcotry based on data from header
    pub fn load_entries(&mut self) {
        let target = &mut self.entries as *mut Entry;

        let entry_size = mem::size_of::<Entry>() as u16;

        let lba: u64 = FAT_START as u64
            + (self.header.reserved_sectors
                + self.header.sectors_per_fat * self.header.fat_count as u16) as u64;

        let size: u16 = entry_size * self.header.dir_entries_count;
        let sectors: u16 = size / self.header.bytes_per_sector;

        unsafe {
            DISK.read(target, lba, sectors);
        }
    }

    //list each entry in root direcotry
    //TODO: add other info like creation_date ecc
    pub fn list_entries(&self) {
        println!("Listing root directory entries:");

        println!("Name          Size          Cluster number");

        for i in 0..ENTRY_COUNT {
            if self.entries[i].name[0] != 0 {
                //print name
                for c in self.entries[i].name {
                    print!("{}", c as char);
                }
                //print size
                let size = self.entries[i].size;
                print!("   {} bytes", size);

                //print cluster
                let cluster = self.entries[i].first_cluster_low;
                print!("     {}", cluster);
                println!();
            }
        }
    }

    pub fn load_table(&mut self) {
        let target = &mut self.table as *mut u16;

        let lba: u64 = FAT_START as u64 + self.header.reserved_sectors as u64;

        //let sectors: u16 = self.header.sectors_per_fat;
        let sectors: u16 = 1;

        unsafe {
            DISK.read(target, lba, sectors);
        }
    }

    pub fn read_file(&mut self, entry: &Entry) {
        let target = &mut self.buffer as *mut u8;

        let data_lba: u64 = FAT_START as u64
            + (self.header.reserved_sectors
                + self.header.sectors_per_fat * self.header.fat_count as u16
                + 32) as u64;
        let lba: u64 = data_lba
            + ((entry.first_cluster_low - 2) * self.header.sectors_per_cluster as u16) as u64;

        let sectors: u16 = self.header.sectors_per_cluster as u16;

        unsafe {
            DISK.read(target, lba, sectors);
        }
    }

    pub fn read_file_to_target(&mut self, entry: &Entry, target: *mut u32) {
        let data_lba: u64 = FAT_START as u64
            + (self.header.reserved_sectors
                + self.header.sectors_per_fat * self.header.fat_count as u16
                + 32) as u64;
        let lba: u64 = data_lba
            + ((entry.first_cluster_low - 2) * self.header.sectors_per_cluster as u16) as u64;

        let sectors: u16 = self.header.sectors_per_cluster as u16;

        unsafe {
            DISK.read(target, lba, sectors);
        }
    }

    pub fn search_file(&self, name: &[char]) -> Entry {
        let mut result = NULL_ENTRY;

        for entry in self.entries {
            let mut found = true;
            let mut i = 0;

            for n in name {
                let mut c = n.clone();

                if c.is_ascii_lowercase() {
                    c = c.to_ascii_uppercase();
                }

                if (c != entry.name[i] as char) && (name[i] != '\0') {
                    found = false;
                }

                i += 1;
            }

            if found {
                result = entry;
            }
        }

        result
    }
}
