org 0x0
bits 16

%define ENDL 0x0D, 0x0A

start:
    mov si, msg_welcome
    call print

.halt:
    cli
    hlt

%include "src/bootloader/print.asm"

msg_welcome: db 'Started kernel', ENDL, 0