<p align="center"><img src="https://user-images.githubusercontent.com/10211171/231185775-eb188a74-a133-45f6-8f2b-21cc1d498a0a.jpg" height=150></p>
<h1 align="center">Felix OS</h1>
<h3 align="center">
x86 operating system
</h3>

## Description

Felix is my attempt at writing an x86 operating system.

It's **written completely from scratch** in Rust and doesn't use any external dependencies.

## Pictures
Felix running in QEMU:<br>
![felix_qemu](https://user-images.githubusercontent.com/10211171/230795939-eab6ab78-5c88-4ece-8dc1-f0e7faca9df9.png)

Felix running on real hardware:<br>
![felix_real](https://user-images.githubusercontent.com/10211171/230796141-b2c62d63-5c4e-4d8b-a9ee-3669bdee48b0.jpg)

## Features

### Bootloader
- boots (you don't say!)
- BIOS compatible (also works on UEFI with CSM enabled)
- Global Descriptor Table loading
- Unreal Mode switching (to use 32bit addresses in 16bit Real Mode)
- kernel copying from disk to protected memory
- 32bit Protected Mode switching
- kernel jumping

### Kernel
- print! macro able to write formatted text to VGA text buffer 
- Interrupt Descriptor Table loading
- CPU exceptions handler
- Programmable Interrupt Controller driver
- keyboard driver
- ATA disk driver
- FAT16 filesystem file read

### Shell
Available commands:
- **help** shows available commands
- **ls** lists root directory entries
- **cat <filename>** displays content of a file

## Building

You can download a pre-built image or you can build it by yourself using Docker.

### Download pre-built image
[![build](https://github.com/mrgian/felix/actions/workflows/docker.yml/badge.svg)](https://github.com/mrgian/felix/actions)

A build is made for every commit.

To download the latest build click on the badge above, then click on the most recent build and download the artifact.

### Build using Docker
First make sure you have Docker installed. Then:

1. Clone the repo `git clone https://github.com/mrgian/felix`
2. Change dir to repo `cd felix`
3. Build the image `docker build -t felix-image .`
4. Run the container `docker run --name felix-container felix-image`
5. Copy build from container to host `docker cp felix-container:/root/felix/build .`

### Build using script
Make sure you have `rustup`,`mtools`,`dosfstools` and `fdisk` installed on your system. Then:

1. Clone the repo `git clone https://github.com/mrgian/felix`
2. Change dir to repo `cd felix`
3. Run build script `./build.sh`

## Running
The final disk image is `build/disk.img`

You can run it in QEMU using this command: `qemu-system-i386 -drive file=build/disk.img,index=0,media=disk,format=raw,if=ide`

Or you can run it on a real x86 computer by copying the disk image to a USB drive using this command: `sudo dd if=build/disk.img of=/dev/sdX status=progress` and then booting from USB.

## Progress
- *22/10/22* - Project start
- *27/01/23* - Bootloader can print to screen
- *31/01/23* - Bootloader can read data from disk to memory
- *01/02/23* - Bootloader can load kernel to memory
- *27/02/23* - Moved to Rust environment using inline assembly
- *01/03/23* - Rewritten kernel loading code in Rust
- *08/03/23* - Implemented println macro
- *20/03/23* - Switch to 32bit protected mode
- *29/03/23* - Basic CPU exception handler
- *30/03/23* - PIC driver
- *06/04/23* - keyboard driver
- *07/04/23* - start working on shell
- *08/04/23* - ATA disk driver
- *09/04/23* - FAT filesystem file read


## Credits
This project is entirely developed by **Gianmatteo Palmieri** ([mrgian](https://github.com/mrgian)).
