#!/bin/bash
make
qemu-system-i386 -fda build/main_floppy.img