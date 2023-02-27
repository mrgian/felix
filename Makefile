ASM=nasm

SRC_DIR=src
BUILD_DIR=build

#with this we can refeer to modules by name, instead of filename
.PHONY: all floppy_image kernel stage1 stage2 clean always


## FLOPPY IMAGE
# Create an empty floppy
# Copy the bootloader
# Copy the kernel to sector 1
floppy_image: $(BUILD_DIR)/floppy.img

$(BUILD_DIR)/floppy.img: boot stage2 kernel
	dd if=/dev/zero of=$(BUILD_DIR)/floppy.img bs=512 count=2880 
#	mkfs.fat -F 12 $(BUILD_DIR)/floppy.img
	dd if=$(BUILD_DIR)/boot.bin of=$(BUILD_DIR)/floppy.img conv=notrunc
#	mcopy -i $(BUILD_DIR)/floppy.img $(BUILD_DIR)/kernel.bin "::kernel.bin"
	dd if=$(BUILD_DIR)/stage2.bin of=$(BUILD_DIR)/floppy.img bs=1 seek=512 conv=notrunc
	
## BOOTLOADER
# Assemble the bootloader
boot: $(BUILD_DIR)/boot.bin

$(BUILD_DIR)/boot.bin: always
	$(ASM) $(SRC_DIR)/bootloader/boot.asm -f bin -o $(BUILD_DIR)/boot.bin

stage2: $(BUILD_DIR)/stage2.bin

$(BUILD_DIR)/stage2.bin: always
	$(ASM) $(SRC_DIR)/bootloader/stage2.asm -f bin -o $(BUILD_DIR)/stage2.bin


## KERNEL
# Assemble the kernel
kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin: always
	$(ASM) $(SRC_DIR)/kernel/main.asm -f bin -o $(BUILD_DIR)/kernel.bin


## ALWAYS
# Make build dir
always:
	mkdir -p $(BUILD_DIR)

## CLEAN
# Clean build dir
clean:
	rm -rf $(BUILD_DIR)/*