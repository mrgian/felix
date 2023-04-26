.PHONY: get-deps
get-deps:
	@brew list util-linux > /dev/null || brew install util-linux
	@brew list e2fsprogs > /dev/null || brew install e2fsprogs
	@brew list mtools > /dev/null || brew install mtools
	@brew list binutils > /dev/null || brew install binutils
	@brew list dosfstools > /dev/null || brew install dosfstools

.PHONY: all
all: get-deps build objcopy
	@dd if=/dev/zero of=build/disk.img bs=67108864 count=1
	@$(shell brew --prefix util-linux)/sbin/sfdisk build/disk.img < disk.layout
	@dd if=build/boot.bin of=build/disk.img conv=notrunc
	@dd if=build/disk.img of=build/partition.img bs=512 skip=36864
	@$(shell brew --prefix dosfstools)/sbin/mkfs.fat -F 16 build/partition.img
	@$(shell brew --prefix mtools)/bin/mcopy -i build/partition.img dante "::dante"
	@$(shell brew --prefix mtools)/bin/mcopy -i build/partition.img lorem "::lorem"
	@$(shell brew --prefix mtools)/bin/mcopy -i build/partition.img build/hello.bin "::hello"
	@dd if=build/partition.img of=build/disk.img bs=512 seek=36864 conv=notrunc
	@rm -rf build/partition.img
	@dd if=build/bootloader.bin of=build/disk.img bs=512 seek=2048 conv=notrunc
	@dd if=build/kernel.bin of=build/disk.img bs=512 seek=4096 conv=notrunc
	@echo "Felix has been successfully built!"

.PHONY: build
build:
	@echo "Building Felix..."
	@cargo build --target=x86_16-felix.json --package=felix-boot
	@cargo build --target=x86_16-felix.json --package=felix-bootloader
	@cargo build --target=x86_32-felix.json --package=felix-kernel
	@cargo build --target=x86_32-felix.json --package=hello

.PHONY: objcopy
objcopy:
	@echo "Copying Felix..."
	@mkdir -p build
	@$(shell brew --prefix binutils)/bin/objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-boot build/boot.bin
	@$(shell brew --prefix binutils)/bin/objcopy -I elf32-i386 -O binary target/x86_16-felix/debug/felix-bootloader build/bootloader.bin
	@$(shell brew --prefix binutils)/bin/objcopy -I elf32-i386 -O binary target/x86_32-felix/debug/felix-kernel build/kernel.bin
	@$(shell brew --prefix binutils)/bin/objcopy -I elf32-i386 -O binary target/x86_32-felix/debug/hello build/hello.bin

.PHONY: clean
clean:
	@echo "Cleaning Felix..."
	@cargo clean
	@rm -rf build

.PHONY: run
run:
	@echo "Running Felix..."
	@qemu-system-i386 -drive file=build/disk.img,index=0,media=disk,format=raw,if=ide
