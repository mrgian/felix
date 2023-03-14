#[path = "disk.rs"]
mod disk;

#[derive(Copy, Clone)]
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

pub fn load_root(address: u16) {
    let lba: u16 = 19; //read from lba 2816 (2880 - 64), last 64 sectors
    let sector_count: u16 = 14; //number of sectors to read

    let disk = disk::DiskReader::from_lba(lba, sector_count, address);

    //actual reading
    disk.load_sectors();
}