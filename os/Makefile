target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin
usr := ../usr/target/$(target)/debug/hello

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run usr

export USER_IMG = ../usr/template/target/$(target)/debug/hello

default: build

usr:
	cd ../usr/template && cargo build && cd ../..

$(usr): usr

build: $(bin)

kernel: $(usr)
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

clean:
	cargo clean && cd ../usr/template && cargo clean && cd ../../os

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
	cargo fmt && cd ../usr/template && cargo fmt && cd ../../os

run: build qemu