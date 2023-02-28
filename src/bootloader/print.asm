mov si, {0:x}

2:
    lodsb
    or al, al
    jz 1f

    mov ah, 0x0e 
    mov bh, 0 
    int 0x10

    jmp 2b
1: