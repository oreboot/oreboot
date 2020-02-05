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

BROKEN := src/mainboard/ast/ast25x0/Makefile src/mainboard/emulation/qemu-armv7/Makefile
# somebody else can figure this out. MAINBOARDS := $(filter-out $(wildcard src/mainboard/*/*/Makefile), $(BROKEN))
MAINBOARDS := \
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
	cargo install cargo-xbuild cargo-binutils
	sudo apt-get install device-tree-compiler pkg-config libssl-dev

update:
	rustup update

clean:
	rm -rf $(wildcard src/mainboard/*/*/target)
