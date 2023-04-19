#!/bin/bash

set -e

echo "Cleaning build directory..."
rm -rf build

echo "Building Felix..."
cargo build --target=x86_16-felix.json --package=felix-boot
cargo build --target=x86_16-felix.json --package=felix-bootloader
cargo build --target=x86_32-felix.json --package=felix-kernel
cargo build --target=x86_32-felix.json --package=hello

echo "Making build directory..."
mkdir build

echo "Objcopy..."
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-boot build/boot.bin
objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-bootloader build/bootloader.bin
objcopy -I elf32-i386 -O binary target/x86_32-felix/debug/felix-kernel build/kernel.bin

echo "Making empty disk image..."
dd if=/dev/zero of=build/disk.img bs=64MiB count=1

echo "Partitioning disk image..."
/sbin/sfdisk build/disk.img < disk.layout

echo "Putting boot sector in disk image..."
dd if=build/boot.bin of=build/disk.img conv=notrunc

echo "Getting main partition from disk image..."
dd if=build/disk.img of=build/partition.img bs=512 skip=36864

echo "Formatting main partition..."
/sbin/mkfs.fat -F 16 build/partition.img

echo "Copying kernel and data to main partition..."
mcopy -i build/partition.img dante "::dante"
mcopy -i build/partition.img lorem "::lorem"

echo "Putting main partition to disk image..."
dd if=build/partition.img of=build/disk.img bs=512 seek=36864 conv=notrunc

echo "Removing temp partition file..."
rm -rf build/partition.img

echo "Putting bootloader..."
dd if=build/bootloader.bin of=build/disk.img bs=512 seek=2048 conv=notrunc

echo "Putting kernel..."
dd if=build/kernel.bin of=build/disk.img bs=512 seek=4096 conv=notrunc

echo "Felix has been successfully built!"