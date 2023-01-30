;bios puts the os in ram at 0x7c00, so tell nasm to calculate addresses from this address
org 0x7c00

;tell nasm to generate 16bit code
bits 16

;define endl as line feed + carriage return
%define ENDL 0x0d, 0x0a

;FAT12 header
;we need to put this because the bootloader will overwrite the first sector of fat12 formatted image
jmp short start
nop

bdb_oem:                    db 'MSWIN4.1'           ; 8 bytes
bdb_bytes_per_sector:       dw 512
bdb_sectors_per_cluster:    db 1
bdb_reserved_sectors:       dw 1
bdb_fat_count:              db 2
bdb_dir_entries_count:      dw 0E0h
bdb_total_sectors:          dw 2880                 ; 2880 * 512 = 1.44MB
bdb_media_descriptor_type:  db 0F0h                 ; F0 = 3.5" floppy disk
bdb_sectors_per_fat:        dw 9                    ; 9 sectors/fat
bdb_sectors_per_track:      dw 18
bdb_heads:                  dw 2
bdb_hidden_sectors:         dd 0
bdb_large_sector_count:     dd 0

;extended boot record
ebr_drive_number:           db 0                    ; 0x00 floppy, 0x80 hdd, useless
                            db 0                    ; reserved
ebr_signature:              db 29h
ebr_volume_id:              db 12h, 34h, 56h, 78h   ; serial number, value doesn't matter
ebr_volume_label:           db 'FELIX      '        ; 11 bytes, padded with spaces
ebr_system_id:              db 'FAT12   '           ; 8 bytes


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

    ;print message
    mov si, message
    call print

    ;just halt the cpu
    hlt

;in some cases other istruction after the end might be executed, this loop prevents that
.halt:
    jmp .halt

; DATA
message: db 'Porcaccio Dio', ENDL, 0

;put all zeros till byte 510, so write 0 for 510-(program size)
times 510-($-$$) db 0

;put 0xaa55 signature as two last bytes of program
dw 0xaa55