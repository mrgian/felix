;converts LBA address (Logical Block Addrress) to CHS address (cylinder, head, sector)
;parameter:
;   ax -> LBA address
;returns:
;   cx (bits 0-5) -> sector
;   cx (bits 6-15) -> cylinder
;   dh -> head 
lba_to_chs:
    push ax
    push dx

    xor dx, dx                          ;dx = 0
    div word [bdb_sectors_per_track]    ;ax = LBA / sectors_per_track
                                        ;dx = LBA % sectors_per_track
    inc dx                              ;dx = (LBA % sectors_per_track) + 1
    mov cx, dx                          ;cx = sector

    xor dx, dx                          ;dx = 0
    div word [bdb_heads]                ;ax = (LBA / sectors_per_track) / heads = cylinder
                                        ;dx = (LBA / sectors_per_track) % heads = head
    
    mov dh, dl                          ;dh = head (because heads number is in the lower part od dx)
    mov ch, al                          ;ch = cylinder (only the lower 8 bits, there are 2 more to put in cl)
    shl ah, 6
    or cl, ah                           ;now we put upper 2 bits of cylinder in cl

    pop ax
    mov dl,al
    pop ax
    ret
    
;Representation of this mess:
;cx         = [  CH  ] [  CL  ]
;cylinder   = XXXXXXXX XX
;sector     =            XXXXXX
