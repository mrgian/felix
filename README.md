# 🐈 Felix

Felix is a wannabe x86 microkernel 

![felix](https://user-images.githubusercontent.com/10211171/216172754-36cc3d1b-fad0-48da-9a58-0991be15c1b5.png)

## Compiling and running
(Only for Linux)

Make sure you have BOCHS installed on your system.

Then just enter `./debug.sh` in a terminal.

The Bochs emulator should start, press "continue" to cotinue with the execution.

The script will take care of everything (assembling, making the image and running)

## Progress
- *22/10/22* - Project start
- *27/01/23* - Bootloader can print to screen
- *31/01/23* - Bootloader can read data from disk to memory
- *01/02/23* - Bootloader can load kernel to memory
- *27/02/23* - Moved to Rust environment using inline assembly
