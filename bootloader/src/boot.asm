.section .boot, "awx"
.global _start
.code16

bdb_oem:                    .byte 0x77, 0x83, 0x87, 0x73, 0x78, 0x52, 0x46, 0x49 # 8 byte "MSWIN4.1"
bdb_bytes_per_sector:       .2byte 512
bdb_sectors_per_cluster:    .byte 1
bdb_reserved_sectors:       .2byte 1
bdb_fat_count:              .byte 2
bdb_dir_entries_count:      .2byte 0x0E0
bdb_total_sectors:          .2byte 2880                
bdb_media_descriptor_type:  .byte 0x0F0              
bdb_sectors_per_fat:        .2byte 9                 
bdb_sectors_per_track:      .2byte 18
bdb_heads:                  .2byte 2
bdb_hidden_sectors:         .4byte 0
bdb_large_sector_count:     .4byte 0

ebr_drive_number:           .byte 0                    
ebr_reserved:               .byte 0                
ebr_signature:              .byte 0x29
ebr_volume_id:              .byte 0x12, 0x34, 0x56, 0x78   
ebr_volume_label:           .byte 0x70, 0x69, 0x76, 0x73, 0x88, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32 # 11 byte "FELIX      "
ebr_system_id:              .byte 0x70, 0x65, 0x84, 0x49, 0x50, 0x32, 0x32, 0x32 # 8 byte "FAT12   "

_start:
    # set data segments to zero
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    # set stack pointer to beginning of program, so it grows before the program
    # the stack grows downwards when you push, so putting the stack after the program would overwrite the program
    # rember that bios loads the program at 0x7c00 in memory, so everything before is empty (not sure about this)
    cld
    mov sp, 0x7c00

    # call main rust function
    call main

# spin to avoid running after the end of the program
spin:
    hlt
    jmp spin

