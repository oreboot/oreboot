help:
	@echo 'Make options:'
	@echo 'firsttime -- for the first time you run make'
	@echo 'update -- to update the install'
	@echo 'format -- to format all files'
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

BROKEN := src/mainboard/ast/ast25x0/Makefile
# somebody else can figure this out. MAINBOARDS := $(filter-out $(wildcard src/mainboard/*/*/Makefile), $(BROKEN))
MAINBOARDS := \
	src/mainboard/emulation/qemu-armv7/Makefile \
	src/mainboard/emulation/qemu-q35/Makefile \
	src/mainboard/emulation/qemu-riscv/Makefile \
	src/mainboard/nuvoton/npcm7xx/Makefile \
	src/mainboard/opentitan/crb/Makefile \
	src/mainboard/sifive/hifive/Makefile \

.PHONY: mainboards $(MAINBOARDS)
mainboards: $(MAINBOARDS)

$(MAINBOARDS):
	cd $(dir $@) && make

firsttime:
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2020-04-22
	rustup override set nightly-2020-04-23
	rustup component add rust-src llvm-tools-preview
	rustup target add riscv64imac-unknown-none-elf
	rustup target add riscv32imc-unknown-none-elf
	rustup target add armv7r-none-eabi
	cargo install --version 0.5.29 cargo-xbuild
	cargo install --version 0.2.0 cargo-binutils
	sudo apt-get install device-tree-compiler pkg-config libssl-dev

update:
	rustup update

# Option used for formatting. If set, the command will only verify if
# formatting is correct (without actually changing the formatting).
# Returns 0 only if all files are properly formatted.
# Usage:
# 	$ make --keep-going format check=true
check ?=

# Makefile does not support recursive wildcard, so we have to handle all depths manually.
CRATES_TO_FORMAT := \
	$(wildcard */Cargo.toml) \
	$(wildcard */*/Cargo.toml) \
	$(wildcard */*/*/Cargo.toml) \
	$(wildcard */*/*/*/Cargo.toml)
$(CRATES_TO_FORMAT):
	cargo fmt --manifest-path $@ -- $(if $(check),--check,)
.PHONY: format $(CRATES_TO_FORMAT)
format: $(CRATES_TO_FORMAT)

clean:
	rm -rf $(wildcard src/mainboard/*/*/target)
