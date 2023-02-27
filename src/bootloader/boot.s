.section .boot, "awx"
.global _start
.code16

# print a string to the screen
# parameters: ds:si that points to the start of the string
print:
    # save si and ax, since we modify them, we need to restore its content after the end of function
    push si
    push ax
    push bx

# loop for each character
.loop:
    lodsb # loads a byte (the next character) from ds:si in the al register
    or al, al # performs bitwise or on al, if al is null sets the zero flag to true, so we can check if we reached end of the string
    jz .done # jumps to done if zero flag is true (reached end of the string)

    # bios interrupts
    # this tells the bios to write content of al to screen
    mov ah, 0x0e # function to write character to tty
    mov bh, 0 # page number
    int 0x10 # bios video category

    jmp .loop # start again

.done:
    # restore bx, ax and si
    pop bx
    pop ax
    pop si
    ret

_start:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    cld

    mov sp, 0x7c00

    # setting video mode to clear the screen
    mov ah, 0
    int 0x10

    # print message
    mov si, _message
    call print

# rust:
#    push dx
#    call first_stage

spin:
    hlt
    jmp spin

