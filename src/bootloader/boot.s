.section .boot, "awx"
.global _start
.code16
.intel_syntax noprefix

_start:
    jmp main

puts:
    push si
    push ax
    push bx

.loop:
    lodsb
    or al, al
    jz .done

    mov ah, 0x0E
    mov bh, 0
    int 0x10

    jmp .loop

.done:
    pop bx
    pop ax
    pop si    
    ret
    
main:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    cld

    mov sp, 0x7c00

    lea si, message
    call puts

spin:
    hlt
    jmp spin

message: .string "Hello world\r\n"

