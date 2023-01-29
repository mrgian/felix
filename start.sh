#!/bin/bash

# Builds os and starts it in a vm
echo "Building Felix..."
make
echo "Starting Felix in a VM..."
qemu-system-i386 -drive file=build/main_floppy.img,index=0,if=floppy,format=raw