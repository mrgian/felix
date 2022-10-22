#!/bin/bash

# Builds os and starts it in a vm
make
qemu-system-i386 -fda build/main_floppy.img