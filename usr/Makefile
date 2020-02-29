target := riscv64imac-unknown-none-elf
mode := debug
rust_src_dir := rust/src/bin
rust_target_dir := rust/target/$(target)/$(mode)
rust_srcs := $(wildcard $(rust_src_dir)/*.rs)
rust_targets := $(patsubst $(rust_src_dir)/%.rs, $(rust_target_dir)/%, $(rust_srcs))
out_dir := build/riscv64
sfsimg := build/usr.img
.PHONY: rust img clean

default: img

rust:
	cd rust && cargo build
	echo targets includes $(rust_targets)
	rm -rf $(out_dir)/rust && mkdir -p $(out_dir)/rust
	rm -f $(sfsimg)
	cp $(rust_targets) $(out_dir)/rust

$(sfsimg): rust
	rcore-fs-fuse --fs sfs $@ $(out_dir) zip

img: $(sfsimg)

fmt:
	cd rust && cargo fmt && cd ..

clean:
	cd rust && cargo clean && cd ..
	rm -rf build/