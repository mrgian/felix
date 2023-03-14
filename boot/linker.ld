ENTRY(_start)

SECTIONS {
    . = 0x500;
    _stack_start = .;

    . = 0x7c00;
    _stack_end = .; /* linker need to know where boot will be when loaded in memory, bios loads boot at 0x7c00 */

    _boot_start = .;

    /* needed to jump to _start after the fat header */
    .jump :
    {
        BYTE(0Xeb) 
        BYTE(0X3c) 
        BYTE(0X90) 
    }

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
    _boot_end = .;

    . = 0x7c00 + 510;

    .magic_number :
    {
        SHORT(0xaa55)
    }

    _bootloader_start = .;
}