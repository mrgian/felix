ENTRY(_start)

SECTIONS {
    . = 0x500;
    _stack_start = .;
    . = 0x7c00;
    _stack_end = .;

    _mbr_start = .;

    /* needed to jump to _start after the fat header */
    .jump :
    {
        BYTE(0Xeb) 
        BYTE(0X3f) 
        BYTE(0X90) 
    }

    .boot :
    {
        *(.boot .boot.*)
    }
    .text :
    {
        *(.text .text.*)
    }
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    .data :
    {
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.got .got.*)
    }
    _mbr_end = .;

    . = 0x7c00 + 510;

    .magic_number :
    {
        SHORT(0xaa55)
    }

    _kernel_start = .;
}