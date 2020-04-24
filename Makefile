help:
	@echo 'Make options:'
	@echo 'firsttime -- for the first time you run make'
	@echo 'update -- to update the install'
	@echo '  # Build a single board'
	@echo '  make VENDOR/BOARD'
	@echo '  # This is equivalent to'
	@echo '  cd src/mainboard/VENDOR/BOARD && make'
	@echo '  # Build all mainboards'
	@echo '  make mainboards'
	@echo '  # Build everything in parallel'
	@echo '  make -j mainboards'
	@echo '  # Build debug mode'
	@echo '  MODE=debug make mainboards'

MAINBOARDS := $(shell cd src/mainboard && find -name Makefile -printf "%h ")

.PHONY: mainboards $(MAINBOARDS)
mainboards: $(MAINBOARDS)

$(MAINBOARDS):
	cd src/mainboard/$@ && make

firsttime:
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	rustup override set nightly
	rustup component add rust-src llvm-tools-preview rustfmt
	rustup target add riscv64imac-unknown-none-elf
	rustup target add riscv32imc-unknown-none-elf
	rustup target add armv7r-none-eabi
	cargo install cargo-xbuild cargo-binutils
	sudo apt-get install device-tree-compiler pkg-config libssl-dev

fmt:
	find -name Cargo.toml | xargs -I{} cargo fmt --manifest-path {}

update:
	rustup update

clean:
	rm -rf $(wildcard src/mainboard/*/*/target)
