#!/bin/bash

# Builds os and starts it in a vm
echo "Building Felix..."
rm -rf build/boot_nasm.bin
rm -rf build/floppy.img
nasm src/bootloader/boot.asm -f bin -o build/boot_nasm.bin
dd if=/dev/zero of=build/floppy.img bs=512 count=2880 
dd if=build/boot_nasm.bin of=build/floppy.img conv=notrunc

echo "Debugging Felix with Bochs..."
bochs -q -f bochs.conf