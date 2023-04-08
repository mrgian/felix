#!/bin/bash

set -e

./build.sh

echo "Debugging with Bochs..."
#bochs -q -f bochs.conf
qemu-system-i386 -hda build/disk.img -M q35