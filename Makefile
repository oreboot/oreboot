help:
	@echo 'Make options:'
	@echo 'firsttime -- for the first time you run make'
	@echo 'update -- to update the install'
	@echo 'format -- to format all files'
	@echo 'checkformat -- to format all files with checking enabled'
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

# Turn them all off. We'll turn them back on to try to get to working tests.
MAINBOARDS := $(wildcard src/mainboard/*/*/Makefile)

# NOTE: These are the host utilities, requiring their own recent Rust version.
RUST_VER := 1.85
BINUTILS_VER := 0.3.6
DPRINT_VER := 0.41.0

CARGOINST := rustup run --install $(RUST_VER) cargo install

.PHONY: $(MAINBOARDS)
mainboards: $(MAINBOARDS)
$(MAINBOARDS):
	make --no-print-directory -C $(dir $@) cibuild

firsttime:
	$(CARGOINST) $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	$(CARGOINST) $(if $(DPRINT_VER),--version $(DPRINT_VER),) dprint

nexttime:
	$(CARGOINST) --force $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	$(CARGOINST) --force $(if $(DPRINT_VER),--version $(DPRINT_VER),) dprint


debiansysprepare:
	sudo apt-get install \
		device-tree-compiler \
		pkg-config \
		libssl-dev \
		llvm-dev \
		libclang-dev \
		clang \
		qemu-system-x86 \
		binutils-riscv64-unknown-elf \
		libudev-dev \

	# -y makes it non-interactive.
	curl https://sh.rustup.rs -sSf | sh -s -- -y

.PHONY: ciprepare debiansysprepare firsttime
ciprepare: debiansysprepare firsttime

update:
	rustup update

# NOTE: do NOT use the cargo command in targets below.
# ALWAYS USE MAKE!

ALLMAKEFILE := \
	$(wildcard payloads/Makefile) \
	$(wildcard payloads/*/Makefile) \
	$(wildcard payloads/*/*/Makefile) \
	$(wildcard payloads/*/*/*/Makefile) \
	$(wildcard payloads/*/*/*/*/Makefile) \
	$(wildcard src/Makefile) \
	$(wildcard src/*/Makefile) \
	$(wildcard src/*/*/Makefile) \
	$(wildcard src/*/*/*/Makefile) \
	$(wildcard src/*/*/*/*/Makefile)

# Ron still doesn't understand this
TEST_ALL_MAKE_DEFAULT := $(patsubst %/Makefile,%/Makefile.default,$(ALLMAKEFILE))
$(TEST_ALL_MAKE_DEFAULT):
	make --no-print-directory -C $(dir $@) default
.PHONY: testdefault $(TEST_ALL_MAKE_DEFAULT)
testdefault: $(TEST_ALL_MAKE_DEFAULT)

.PHONY: format
format:
	dprint fmt

.PHONY: checkformat
checkformat:
	dprint check

# TODO: Remove write_with_newline
CRATES_TO_CLIPPY := $(patsubst %/Makefile,%/Makefile.clippy,$(ALLMAKEFILE))
$(CRATES_TO_CLIPPY):
	make --no-print-directory -C $(dir $@) ciclippy
.PHONY: clippy $(CRATES_TO_CLIPPY)
clippy: $(CRATES_TO_CLIPPY)

# convenience target: this should be the full ci flow

checkandbuildall: ciprepare clippy checkformat test mainboards
	echo "Done CI!"

clean:
	rm -rf $(TOP)/target
