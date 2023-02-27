#!/bin/bash

# Builds os and starts it in a vm
echo "Cleaning build directory..."
rm -rf build/*
echo "Building Felix..."
cargo build

echo "Making boot image..."
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix build/boot.bin
dd if=/dev/zero of=build/floppy.img bs=512 count=2880 
dd if=build/boot.bin of=build/floppy.img conv=notrunc

echo "Debugging Felix with Bochs..."
bochs -q -f bochs.conf