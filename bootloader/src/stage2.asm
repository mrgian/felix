org 0x7e00
bits 16

%define ENDL 0x0D, 0x0A

start:
    mov si, msg_welcome
    call print

.halt:
    cli
    hlt

%include "src/bootloader/print.asm"

msg_welcome: db 'Stage 2', ENDL, 0