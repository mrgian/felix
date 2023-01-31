;bios puts the os in ram at 0x7c00, so tell nasm to calculate addresses from this address
org 0x7c00

;tell nasm to generate 16bit code
bits 16

;define endl as line feed + carriage return
%define ENDL 0x0d, 0x0a

;we need to put this because the bootloader will overwrite the first sector of fat12 formatted image
%include "src/bootloader/header.asm"

;there are other functions before main, with this we keep main the entry point
start:
    jmp main

%include "src/bootloader/print.asm"

main:
    ;setup data segments to zero
    ;set ax to zero and then ds and es to ax, because you can't set ds and es directly in 16 bit mode
    mov ax, 0
    mov ds, ax
    mov es, ax

    ;set stack segment to zero
    mov ss, ax

    ;set stack pointer to beginning of program, so it grows before the program
    ;the stack grows downwards when you push, so putting the stack after the program would overwrite the program
    ;rember that bios loads the program at 0x7c00 in memory, so everything before is empty (not sure about this)
    mov sp, 0x7c00

    ;setting video mode to clear the screen
    mov ah, 0
    int 0x10

    ;read from floppy
    mov [ebr_drive_number], dl
    mov ax, 1               ;lba = 1
    mov cl, 1               ;read one sector
    mov bx, 0x7e00          ;where to read data
    call read_disk

    ;print message
    mov si, message
    call print

    ;just halt the cpu
    hlt

%include "src/bootloader/disk.asm"

;in some cases other istruction after the end might be executed, this loop prevents that
;.halt:
;    jmp .halt

; DATA
message: db 'Porcaccissimo Dio', ENDL, 0
message_read_failed: db 'Read failed!', ENDL, 0

;put all zeros till byte 510, so write 0 for 510-(program size)
times 510-($-$$) db 0

;put 0xaa55 signature as two last bytes of program
dw 0xaa55