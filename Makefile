target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run

build: $(bin)

kernel:
	cargo +nightly build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

clean:
	cargo +nightly clean

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

run: build qemu

push:
	git push hub master
	git push lab master