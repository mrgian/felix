#!/bin/bash

set -e

./build.sh

echo "Debugging with Bochs..."
bochs -q -f bochs.conf
#qemu-system-i386 -drive file=build/disk.img,index=0,media=disk,format=raw,if=ide