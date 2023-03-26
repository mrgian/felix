#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8,
    offset_high: u16,
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,                                  //idt size                  
    offset: *const InterruptDescriptorTable,    //pointer to idt
}