ENTRY(_start)

SECTIONS {
    . = 0x00500000;

    .start_marker :
    {
        LONG(0xB16B00B5)
    }

    _app_start = .;

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

    .end_marker :
    {
        SHORT(0xdead)
    }
}