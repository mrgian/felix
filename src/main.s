#bios puts the os in ram at 0x7c00, so tell nasm to calculate addresses from this address
org 0x7c00

#tell nasm to generate 16bit code
bits 16

main:
    #setup data segments to zero
    #set ax to zero and then ds and es to ax, because you can't set ds and es directly in 16 bit mode
    mov ax, 0
    mov ds, ax
    mov es, ax

    #set stack segment to zero
    mov ss, ax

    #set stack pointer to beginning of program
    mov sp, 0x7c00

    #just halt the cpu
    hlt

#in some cases other istruction after the end might be executed, this loop prevents that
halt:
    jmp .halt

#put all zeros till byte 510, so write 0 for 510-(program size)
times 510-($-$$) db 0

#put 0xaa55 signature as two last bytes of program
dw 0xaa55