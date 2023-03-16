.section .boot, "awx"
.global _start
.code16

_start:
    # set data segments to zero
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    # set stack pointer to beginning of program, so it grows before the program
    # the stack grows downwards when you push, so putting the stack after the program would overwrite the program
    # rember that bios loads the program at 0x7c00 in memory, so everything before is empty (not sure about this)
    cld
    mov sp, 0x7c00

    # call main rust function
    call main

# spin to avoid running after the end of the program
spin:
    hlt
    jmp spin

