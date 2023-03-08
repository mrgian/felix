# 🐈 Felix

Felix is (not yet) an x86 operating system.<br>
It's **written completely from scratch** in Rust and Assembly and does't use any external dependecies.

![felix](https://user-images.githubusercontent.com/10211171/216172754-36cc3d1b-fad0-48da-9a58-0991be15c1b5.png)

## Compiling and debugging
(Only for Linux)

Make sure you have the Bochs emulator and Rust installed on your system.

Then run the debug script `./debug.sh`.

The script will take care of everything (compiling, making the image and running the debugger)

The Bochs emulator should start, press "continue" to cotinue with the execution.

## Progress
- *22/10/22* - Project start
- *27/01/23* - Bootloader can print to screen
- *31/01/23* - Bootloader can read data from disk to memory
- *01/02/23* - Bootloader can load kernel to memory
- *27/02/23* - Moved to Rust environment using inline assembly
- *01/03/23* - Rewritten kernel loading code in Rust
