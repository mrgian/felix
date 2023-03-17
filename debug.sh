#!/bin/bash

set -e

./build.sh

echo "Debugging with Bochs..."
bochs -q -f bochs.conf
#qemu-system-i386 -drive id=disk,file=build/disk.img,if=none,format=raw -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0