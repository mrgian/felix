#!/bin/bash

# Builds os and debugs it in Bochs
echo "Cleaning build directory..."
rm -rf build
echo "Building Felix..."
cargo install cargo-binutils
cargo build

echo "Making boot image..."
mkdir build

#this is needed to shrink the bootloader to 512 bytes
cargo objcopy --bin felix-bootloader -- -I elf32-i386 -O binary build/boot.bin
cargo objcopy --bin felix-kernel -- -I elf32-i386 -O binary build/kernel.bin

#create the disk image
#306 cylinders, 4 heads, 17 sectors per track => 20808 sectors in total
dd if=/dev/zero of=build/disk.img bs=512 count=20808 

#put the bootloader in first 512 bytes of disk...
dd if=build/boot.bin of=build/disk.img conv=notrunc

#and the kernel in the remaining space
dd if=build/kernel.bin of=build/disk.img bs=1 seek=512 conv=notrunc

echo "Debugging Felix with Bochs..."
#bochs -q -f bochs.conf
qemu-system-i386 -drive id=disk,file=build/disk.img,if=none,format=raw -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0