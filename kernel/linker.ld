ENTRY(_start)

SECTIONS {
    . = 0x7e00 + 16384;

    _kernel_start = .;

    .start : {
        *(.start)
    }
    .text : {
        *(.text .text.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }

    /* in this way the kernel size is exactly 16384 bytes (32 sectors) */
    . = _kernel_start + 0x4000 - 2;
    .end_marker :
    {
        SHORT(0xdead)
    }
}