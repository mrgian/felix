ASM=nasm
CC16=/usr/bin/watcom/binl/wcc
LD16=/usr/bin/watcom/binl/wlink
CFLAGS16=-4 -d3 -s -wx -ms -zl -zq 

SRC_DIR=src
BUILD_DIR=build

SOURCES_C=$(wildcard *.c)
SOURCES_ASM=$(wildcard *.asm)
OBJECTS_C=$(patsubst %.c, $(BUILD_DIR)/stage2/c/%.obj, $(SOURCES_C))
OBJECTS_ASM=$(patsubst %.asm, $(BUILD_DIR)/stage2/asm/%.obj, $(SOURCES_ASM))

#with this we can refeer to modules by name, instead of filename
.PHONY: all floppy_image kernel boot stage2 clean always


## FLOPPY IMAGE
# Create an empty floppy
# Copy the bootloader
# Copy the kernel to sector 1
floppy_image: $(BUILD_DIR)/floppy.img

$(BUILD_DIR)/floppy.img: boot stage2 kernel
	dd if=/dev/zero of=$(BUILD_DIR)/floppy.img bs=512 count=2880 
	dd if=$(BUILD_DIR)/boot.bin of=$(BUILD_DIR)/floppy.img conv=notrunc
	dd if=$(BUILD_DIR)/stage2.bin of=$(BUILD_DIR)/floppy.img bs=1 seek=512 conv=notrunc
	
## BOOT (stage 1)
boot: $(BUILD_DIR)/boot.bin

$(BUILD_DIR)/boot.bin: always
	$(ASM) $(SRC_DIR)/bootloader/boot/boot.asm -f bin -o $(BUILD_DIR)/boot.bin

## STAGE 2
stage2: $(BUILD_DIR)/stage2.bin

$(BUILD_DIR)/stage2.bin: $(OBJECTS_ASM) $(OBJECTS_C)
	$(LD16) NAME $(BUILD_DIR)/stage2.bin FILE \{ $(OBJECTS_ASM) $(OBJECTS_C) \} OPTION MAP=$(BUILD_DIR)/stage2.map @linker.lnk

$(BUILD_DIR)/stage2/c/%.obj: %.c always
	$(CC16) $(CFLAGS16) -fo=$@ $<

$(BUILD_DIR)/stage2/asm/%.obj: %.asm always
	$(ASM) $(ASMFLAGS) -o $@ $<

## KERNEL
# Assemble the kernel
kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin: always
	$(ASM) $(SRC_DIR)/kernel/main.asm -f bin -o $(BUILD_DIR)/kernel.bin


## ALWAYS
# Make build dir
always:
	mkdir -p $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)/stage2/c
	mkdir -p $(BUILD_DIR)/stage2/asm

## CLEAN
# Clean build dir
clean:
	rm -rf $(BUILD_DIR)/*
