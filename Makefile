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

BROKEN := \
	src/mainboard/ast/ast25x0/Makefile \
	src/mainboard/nuvoton/npcm7xx/Makefile \
	src/mainboard/emulation/qemu-armv7/Makefile \

MAINBOARDS := $(filter-out $(BROKEN), $(wildcard src/mainboard/*/*/Makefile))

TOOLCHAIN_VER := nightly-2020-04-22
XBUILD_VER := 0.5.29
BINUTILS_VER := 0.2.0

.PHONY: mainboards $(MAINBOARDS)
mainboards: $(MAINBOARDS)

$(MAINBOARDS):
	cd $(dir $@) && make

firsttime:
	rustup override set $(TOOLCHAIN_VER)
	rustup component add rust-src llvm-tools-preview rustfmt clippy
	rustup target add riscv64imac-unknown-none-elf
	rustup target add riscv32imc-unknown-none-elf
	rustup target add armv7r-none-eabi
	cargo install $(if $(XBUILD_VER),--version $(XBUILD_VER),) cargo-xbuild
	cargo install $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils

debiansysprepare:
	sudo apt-get install device-tree-compiler pkg-config libssl-dev llvm-dev libclang-dev clang
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $(TOOLCHAIN_VER)

.PHONY: ciprepare debiansysprepare firsttime
ciprepare: debiansysprepare firsttime

update:
	rustup update

# Option used for formatting. If set, the command will only verify if
# formatting is correct (without actually changing the formatting).
# Returns 0 only if all files are properly formatted.
# Usage:
# 	$ make --keep-going format check=true
check ?=

# Makefile does not support recursive wildcard, so we have to handle all depths manually.
CRATES := \
	$(wildcard */Cargo.toml) \
	$(wildcard */*/Cargo.toml) \
	$(wildcard */*/*/Cargo.toml) \
	$(wildcard */*/*/*/Cargo.toml)

CRATES_TO_FORMAT := $(patsubst %/Cargo.toml,%/Cargo.toml.format,$(CRATES))
$(CRATES_TO_FORMAT):
	cd $(dir $@) && cargo fmt -- $(if $(check),--check,)
.PHONY: format $(CRATES_TO_FORMAT)
format: $(CRATES_TO_FORMAT)

BROKEN_CRATES_TO_TEST := \
	src/arch/arm/armv7/Cargo.toml \
	src/arch/riscv/rv32/Cargo.toml \
	src/arch/riscv/rv64/Cargo.toml \
	src/cpu/armltd/cortex-a9/Cargo.toml \
	src/cpu/lowrisc/ibex/Cargo.toml \
	src/mainboard/amd/romecrb/Cargo.toml \
	src/mainboard/ast/ast25x0/Cargo.toml \
	src/mainboard/emulation/qemu-armv7/Cargo.toml \
	src/mainboard/emulation/qemu-q35/Cargo.toml \
	src/mainboard/emulation/qemu-riscv/Cargo.toml \
	src/mainboard/nuvoton/npcm7xx/Cargo.toml \
	src/mainboard/opentitan/crb/Cargo.toml \
	src/mainboard/sifive/hifive/Cargo.toml \
	src/soc/aspeed/ast2500/Cargo.toml \
	src/soc/opentitan/earlgrey/Cargo.toml \
	src/soc/sifive/fu540/Cargo.toml \
	src/vendorcode/fsp/coffeelake/Cargo.toml \

CRATES_TO_TEST := $(patsubst %/Cargo.toml,%/Cargo.toml.test,$(filter-out $(BROKEN_CRATES_TO_TEST),$(CRATES)))
$(CRATES_TO_TEST):
	cd $(dir $@) && cargo test
.PHONY: test $(CRATES_TO_TEST)
test: $(CRATES_TO_TEST)

# TODO: Fix payloads crate and remove "-A clippy::module-inception" exception.
CRATES_TO_CLIPPY := $(patsubst %/Cargo.toml,%/Cargo.toml.clippy,$(filter-out $(BROKEN_CRATES_TO_TEST),$(CRATES)))
$(CRATES_TO_CLIPPY):
	cd $(dir $@) && cargo clippy -- -D warnings -A clippy::module-inception
.PHONY: clippy $(CRATES_TO_CLIPPY)
clippy: $(CRATES_TO_CLIPPY)

clean:
	rm -rf $(wildcard src/mainboard/*/*/target)
