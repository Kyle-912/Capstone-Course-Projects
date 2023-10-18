submission:
	@echo "Creating binary image files for submission"
	as -o x86/outputs/x86-bootloader.o x86/src/x86-bootloader.s
	ld -Ttext=0x7C00 -e _start -o x86/outputs/x86-bootloader.elf x86/outputs/x86-bootloader.o
	objcopy -O binary x86/outputs/x86-bootloader.elf i386_image.bin

	arm-none-eabi-as -march=armv3 -o arm/outputs/arm-bootloader.o arm/src/arm-bootloader.s
	arm-none-eabi-ld -Ttext=0x0 -e _start -o arm/outputs/arm-bootloader.elf arm/outputs/arm-bootloader.o
	arm-none-eabi-objcopy -O binary --only-section=.text arm/outputs/arm-bootloader.elf akita_image.bin

clean_x86:
	@echo "Removing build files..."
	-rm -rf x86/outputs/*

loader_x86: x86/src/x86-bootloader.s
	@echo "Assembling bootloader code into binary image..."
	as -o x86/outputs/x86-bootloader.o x86/src/x86-bootloader.s
	ld -Ttext=0x7C00 -e _start -o x86/outputs/x86-bootloader.elf x86/outputs/x86-bootloader.o
	objcopy -O binary x86/outputs/x86-bootloader.elf x86/outputs/x86-bootloader.bin

qemu_i386:
	@echo "Building binary and running in QEMU..."
	make clean_x86
	make loader_x86
	qemu-system-i386 -cpu 486 -nographic -device loader,file=x86/assets/lemon_log.bin,addr=0x500 -hda x86/outputs/x86-bootloader.bin

qemu_i386_debug:
	@echo "Building binary and running in QEMU(debug)..."
	make clean_x86
	make loader_x86
	qemu-system-i386 -cpu 486 -nographic -s -S -device loader,file=x86/assets/lemon_log.bin,addr=0x500 -hda x86/outputs/x86-bootloader.bin

clean_arm:
	@echo "Removing build files..."
	-rm -rf arm/outputs/*

loader_arm:
	@echo "Assembling bootloader code into binary image..."
	arm-none-eabi-as -march=armv3 -o arm/outputs/arm-bootloader.o arm/src/arm-bootloader.s
	arm-none-eabi-ld -Ttext=0x0 -e _start -o arm/outputs/arm-bootloader.elf arm/outputs/arm-bootloader.o
	arm-none-eabi-objcopy -O binary --only-section=.text arm/outputs/arm-bootloader.elf arm/outputs/arm-bootloader.bin

qemu_akita:
	@echo "Building binary and running in QEMU..."
	make clean_arm
	make loader_arm
	qemu-system-arm -M akita -nographic -device loader,file=arm/assets/lemon_log.bin,addr=0xA3F00000 -device loader,file=arm/outputs/arm-bootloader.bin,addr=0x0

qemu_akita_debug:
	@echo "Building binary and running in QEMU(debug)..."
	make clean_arm
	make loader_arm
	qemu-system-arm -M akita -nographic -s -S -device loader,file=arm/assets/lemon_log.bin,addr=0xA3F00000 -device loader,file=arm/outputs/arm-bootloader.bin,addr=0x0
