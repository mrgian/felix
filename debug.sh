#!/bin/bash

# Builds os and starts it in a vm
echo "Cleaning build directory..."
rm -rf build
echo "Building Felix..."
cargo install cargo-binutils
cargo build

echo "Making boot image..."
mkdir build
cargo objcopy --bin felix-bootloader -- -I elf32-i386 -O binary build/boot.bin
cargo objcopy --bin felix-kernel -- -I elf32-i386 -O binary build/kernel.bin
dd if=/dev/zero of=build/disk.img bs=512 count=20808 
dd if=build/boot.bin of=build/disk.img conv=notrunc
dd if=build/kernel.bin of=build/disk.img bs=1 seek=512 conv=notrunc

echo "Debugging Felix with Bochs..."
bochs -q -f bochs.conf
#qemu-system-i386 -drive file=build/disk.img,index=0,if=floppy,format=raw