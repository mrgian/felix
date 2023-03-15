<p align="center"><img src="https://user-images.githubusercontent.com/10211171/223741548-45ed1c96-a1da-40de-8544-8e10e4ddb072.png" height=100></p>
<h1 align="center">Felix OS</h1>
<h3 align="center">
(not yet) an x86 operating system.
</h3>

## Description

Felix is my attempt at writing an x86 operating system.

It's **written completely from scratch** in Rust and Assembly and doesn't use any external dependecies.

![felix2](https://user-images.githubusercontent.com/10211171/223734499-15768aff-6d6f-4013-9fb5-3e75022a907e.png)<br>
*Felix running in Bochs emulator*

![output](https://user-images.githubusercontent.com/10211171/223737198-9aa156ca-1c57-4db5-932d-e999a1471dc0.gif)<br>
*Felix running on real hardware*

## Compiling and debugging
(Only for Linux)

Make sure you have the Bochs emulator and Rust installed on your system, then run the debug script:

`./debug.sh`

after compiling the Bochs emulator should start, press "continue" to cotinue with the execution.

The script takes care of everything (compiling, making the image and running the debugger).

## Versions
### Felix 0.1.0
**Bootloader:**
- BIOS compatible (also works on UEFI with emulated BIOS)
- can boot
- can load kernel to memory
- can execute kernel

**Kernel:**
- has a _println_ macro able to format any type of data

## Progress
- *22/10/22* - Project start
- *27/01/23* - Bootloader can print to screen
- *31/01/23* - Bootloader can read data from disk to memory
- *01/02/23* - Bootloader can load kernel to memory
- *27/02/23* - Moved to Rust environment using inline assembly
- *01/03/23* - Rewritten kernel loading code in Rust
- *08/03/23* - Implemented println macro
