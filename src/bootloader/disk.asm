floppy_error:
    mov si, message_read_failed
    call print
    jmp wait_key_and_reboot

wait_key_and_reboot:
    mov ah, 0
    int 0x16                ;waits for keypress
    jmp 0xfffffff0          ;jump to first instruction of bios (reboot the system)

.halt:
    cli                     ;disable interrupts
    htl

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

;reads sectors from a disk
;parameters:
;   ax -> LBA address
;   cl -> number of sectors to read
;   dl -> drive number
;   es:bx -> memory address where to store read data
read_disk:
    push ax
    push bx
    push cx
    push dx
    push di

    push cx                 ;save cl
    call lba_to_chs         ;calculate chs
    pop ax                  ;al = number of sectors to read

    mov ah, 0x02
    mov di, 3

.retry:
    pusha
    stc
    int 0x13
    jnc .done

    ;if fails
    popa
    call disk_reset

    dec di
    test di, di
    jnz .retry

.fail:
    jmp floppy_error

.done:
    popa

    pop di
    pop dx
    pop cx
    pop bx
    pop ax

disk_reset:
    pusha
    mov ah, 0
    stc
    int 0x13
    jc floppy_error
    popa
    ret