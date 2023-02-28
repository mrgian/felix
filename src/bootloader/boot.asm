.section .boot, "awx"
.global _start
.code16

_start:
    jmp main
    
main:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    cld

    mov sp, 0x7c00

    call print

spin:
    hlt
    jmp spin

