#!/bin/bash

# Builds os and starts it in a vm
echo "Building Felix..."
make
echo "Debugging Felix with Bochs..."
#qemu-system-i386 -drive file=build/floppy.img,index=0,if=floppy,format=raw
bochs -q -f bochs.conf