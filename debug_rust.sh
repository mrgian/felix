#!/bin/bash

# Builds os and starts it in a vm
echo "Building Felix..."
rm -rf build/boot_rust.bin
rm -rf build/floppy.img
cargo build
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix build/boot_rust.bin
dd if=/dev/zero of=build/floppy.img bs=512 count=2880 
dd if=build/boot_rust.bin of=build/floppy.img conv=notrunc

echo "Debugging Felix with Bochs..."
bochs -q -f bochs.conf