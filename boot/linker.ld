ENTRY(_start)

SECTIONS {
    . = 0x500;
    _stack_start = .;

    . = 0x7c00;
    _stack_end = .; /* linker need to know where boot will be when loaded in memory, bios loads boot at 0x7c00 */

    /* boot.asm section */
    .boot :
    {
        *(.boot .boot.*)
    }

    /* code section */
    .text :
    {
        *(.text .text.*)
    }

    /* read-only data, es. string costants */
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

    . = 0x7c00 + 0x1b8;

    .diskid : 
    {
        BYTE(0Xfb) 
        BYTE(0X00) 
        BYTE(0Xb1)
        BYTE(0Xe5) 
    }

    .reserved : 
    {
        BYTE(0X00) 
        BYTE(0X00) 
    }

    .first_table : 
    {
        BYTE(0X80) /* drive attribute */

        BYTE(0X20)
        BYTE(0X21) 
        BYTE(0X00) /* chs address of partition start */

        BYTE(0X00) /* partition type */

        BYTE(0X41)
        BYTE(0X01) 
        BYTE(0X00) /* chs address of last partition */

        BYTE(0X00)
        BYTE(0X08) 
        BYTE(0X00)
        BYTE(0X00) /* lba of partition start */

        BYTE(0X00)
        BYTE(0X08) 
        BYTE(0X00)
        BYTE(0X00) /* number of sectors */
    }

    .first_table : 
    {
        BYTE(0X00) /* drive attribute */

        BYTE(0X41)
        BYTE(0X02) 
        BYTE(0X00) /* chs address of partition start */

        BYTE(0X06) /* partition type */

        BYTE(0X8a)
        BYTE(0X08) 
        BYTE(0X82) /* chs address of last partition */

        BYTE(0X00)
        BYTE(0X10) 
        BYTE(0X00)
        BYTE(0X00) /* lba of partition start */

        BYTE(0X00)
        BYTE(0Xf0) 
        BYTE(0X1f)
        BYTE(0X00) /* number of sectors */
    }

    . = 0x7c00 + 0x1fe;

    .magic_number :
    {
        SHORT(0xaa55)
    }

    _bootloader_start = .;
}