#[path = "disk.rs"]
mod disk;
use disk::DiskReader;

use core::ptr;

//Link to bible: https://wiki.osdev.org/FAT#Implementation_Details

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Header {
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
    header: Header,
    entries: [Entry; 224], //TODO: entries count is hardcoded but it shouldn't
}

impl FatDriver {
    pub fn new() -> Self {
        Self {
            header: Header::default(),
            entries: [Entry::default(); 224], //TODO: entries count is hardcoded but it shouldn't
        }
    }

    pub fn load_header(&self) {
        let address = ptr::addr_of!(self.header) as u16;

        let lba: u16 = 0;
        let sector_count: u16 = 1;
        let disk = DiskReader::from_lba(lba, sector_count, address);

        disk.load_sectors();
    }

    pub fn load_entries(&self) {
        let address = ptr::addr_of!(self.entries) as u16;

        let lba: u16 = self.header.reserved_sectors + self.header.sectors_per_fat * self.header.fat_count as u16;
        let size: u16 = 32 * self.header.dir_entries_count; //TODO: entries size (32) is hardcoded but it shouldn't
        let sector_count: u16 = size / self.header.bytes_per_sector;

        let disk = DiskReader::from_lba(lba, sector_count, address);

        disk.load_sectors();
    }

    pub fn list_entries(&self) {
        println!("Listing root directory entries:");

        println!();

        for e in 0..224 { //TODO: entries count is hardcoded but it shouldn't
            let name = self.entries[e].name;
            if name[0] != 0 {
                for c in 0..11 { //TODO: name lenght is hardcoded but it shouldn't
                    print!("{}", name[c] as char);
                }
                println!();
            }
        }
    }
}
