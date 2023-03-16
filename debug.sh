#!/bin/bash

# Builds os and debugs it in Bochs
echo "Cleaning build directory..."
rm -rf build
echo "Building Felix..."
cargo build

echo "Making boot image..."
mkdir build

#this is needed to shrink the bootloader to 512 bytes
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-boot build/boot.bin
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-bootloader build/bootloader.bin
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-kernel build/kernel.bin

#create empty disk image
dd if=/dev/zero of=build/disk.img bs=1GiB count=1

#partition disk
sfdisk build/disk.img < disk.layout

#put the boot sector in first 512 bytes of disk
dd if=build/boot.bin of=build/disk.img conv=notrunc

#mount main partition
sudo losetup -d /dev/loop0
sudo losetup --offset $((512*4096)) --show --find build/disk.img

#format main partition
sudo mkfs.fat -F 16 /dev/loop0

put bootloader in boot partition
dd if=build/bootloader.bin of=build/disk.img bs=512 seek=2048 conv=notrunc

#copy kernel and data
sudo mcopy -i /dev/loop0 build/kernel.bin "::kernel.bin"
sudo mcopy -i /dev/loop0 test1.txt "::test1.txt"
sudo mcopy -i /dev/loop0 test2.txt "::test2.txt"

echo "Debugging Felix with Bochs..."
#bochs -q -f bochs.conf
qemu-system-i386 -drive id=disk,file=build/disk.img,if=none,format=raw -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0