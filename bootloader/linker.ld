ENTRY(_start)

SECTIONS {
    . = 0x7c00 + 512; /* 0x7e00, just after the boot */

    _bootloader_start = .;

    .start : {
        *(.start)
    }

    /* code section */
    .text : {
        *(.text .text.*)
    }

    /* static variables section */
    .bss : {
        *(.bss .bss.*)
    }

    /* read-only data, like consts */
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }

    /* contains info about exceptions */
    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }

    /* in this way the bootloader size is exactly 64 sectors) */
    . = _bootloader_start + 0x8000 - 2;
    .end_marker :
    {
        SHORT(0xdead)
    }

}