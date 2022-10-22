org 0x7C00 ;bios puts the os in ram at 0x7c00, so tell nasm to calculate addresses from this address

bits 16 ;tell nasm to generate 16bit code

main:
    hlt ;just halt the cpu

.halt: ;in some cases might execute other istruction after the end, this loop prevents that
    jmp .halt

times 510-($-$$) db 0 ;put all zeros till byte 510, so write 0 for 510-(program size)
dw 0AA55h ;put 0xaa55 signature as two last bytes of program