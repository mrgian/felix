mov si, {0:x} # moves given string address to si

2:
    lodsb # loads a byte (next character) from si to al register
    or al, al # bitwise or on al, if al is null sets zf to true
    jz 1f # if zf is true jump to end

    # bios interrupts
    # this tells the bios to write content of al to screen
    mov ah, 0x0e 
    mov bh, 0 
    int 0x10

    jmp 2b # start again
1: