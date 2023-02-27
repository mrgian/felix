#!/bin/bash

# Builds os and starts it in a vm
echo "Cleaning build directory..."
rm -rf build
echo "Building Felix..."
cargo install cargo-binutils
cargo build

echo "Making boot image..."
mkdir build
cargo objcopy -- -I elf32-i386 -O binary build/boot.bin
dd if=/dev/zero of=build/floppy.img bs=512 count=2880 
dd if=build/boot.bin of=build/floppy.img conv=notrunc

echo "Debugging Felix with Bochs..."
bochs -q -f bochs.conf