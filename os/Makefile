target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin
usr := ../usr/build/usr.img

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run usr

export USER_IMG = $(usr)

default: build

usr:
	cd ../usr && make && cd ..

$(usr): usr

build: $(bin)

kernel: $(usr)
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

clean:
	cargo clean && cd ../usr && make clean && cd ..

qemu: build
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-device loader,file=$(bin),addr=0x80200000

gdb-server: build
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-s \
		-S \
		-device loader,file=$(bin),addr=0x80200000

gdb: build
	riscv64-unknown-elf-gdb $(kernel)

fmt:
	cargo fmt && cd ../usr/rust && cargo fmt && cd ../../os

run: build qemu