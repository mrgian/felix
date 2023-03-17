#[path = "disk.rs"]
mod disk;
use disk::DiskReader;

use core::mem;
use core::ptr;

const ENTRY_COUNT: u16 = 512;
const FAT_START: u16 = 4096;

//Link to bible: https://wiki.osdev.org/FAT#Implementation_Details

//FAT12 header
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
}

impl Default for Header {
    //init header to all zeros
    fn default() -> Header {
        Header {
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
        }
    }
}

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

impl Default for Entry {
    //init entry to all zeros
    fn default() -> Entry {
        Entry {
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
        }
    }
}

pub struct FatDriver {
    pub header: Header,
    pub entries: [Entry; ENTRY_COUNT as usize], //the root directory is an array of file entries
}

impl FatDriver {
    //init empty header and entries to allocate memory
    pub fn new() -> Self {
        Self {
            header: Header::default(),
            entries: [Entry::default(); ENTRY_COUNT as usize],
        }
    }

    //get header address and overwrite that mem location with data from boot sector
    pub fn load_header(&self) {
        let address = ptr::addr_of!(self.header) as u16;

        let lba: u64 = FAT_START as u64;
        let sectors: u16 = 1;

        let mut disk = DiskReader::new(lba, address);

        disk.read_sectors(sectors);
    }

    //get entries array address and overwrite that mem location with data from root directory
    //calculate size and position of root direcotry based on data from header
    pub fn load_entries(&self) {
        let address = ptr::addr_of!(self.entries) as u16;

        let entry_size = mem::size_of::<Entry>() as u16;

        let lba: u64 = FAT_START as u64
            + (self.header.reserved_sectors
                + self.header.sectors_per_fat * self.header.fat_count as u16) as u64;
        let size: u16 = entry_size * self.header.dir_entries_count;
        let sectors: u16 = size / self.header.bytes_per_sector;

        let mut disk = DiskReader::new(lba, address);

        disk.read_sectors(sectors);
    }

    //list each entry in root direcotry
    //TODO: add other info like creation_date ecc
    pub fn list_entries(&self) {
        println!("Listing root directory entries:");

        println!();

        //NOTE: if i scan to 512 it doesn't work, maybe stack is too small to contain all entries 
        for i in 0..64 {
            if self.entries[i].name[0] != 0 {
                for c in self.entries[i].name {
                    print!("{}", c as char);
                }
                println!();
            }
        }

        /*for entry in self.entries {
            if entry.name[0] != 0 {
                for c in entry.name {
                    print!("{}", c as char);
                }
                println!();
            }
        }*/
    }
}
