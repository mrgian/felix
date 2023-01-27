ASM=nasm

SRC_DIR=src
BUILD_DIR=build

# take main.bin and copy it in a 14.4MB file (a floppy disk)
$(BUILD_DIR)/main_floppy.img: $(BUILD_DIR)/main.bin
	cp $(BUILD_DIR)/main.bin $(BUILD_DIR)/main_floppy.img
	truncate -s 1440k $(BUILD_DIR)/main_floppy.img

# create build dir if not exists
# assemble main.asm
$(BUILD_DIR)/main.bin: $(SRC_DIR)/main.s
	mkdir -p $(BUILD_DIR)
	$(ASM) $(SRC_DIR)/main.s -f bin -o $(BUILD_DIR)/main.bin
