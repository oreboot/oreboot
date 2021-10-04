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

TOOLCHAIN_VER := $(shell grep channel rust-toolchain.toml | grep -e '".*"' -o)
BINUTILS_VER := 0.3.2
STACK_SIZES_VER := 0.4.0

.PHONY: mainboards $(MAINBOARDS)
mainboard: $(MAINBOARDS)
$(MAINBOARDS):
	cd $(dir $@) && make cibuild

firsttime:
	cargo install $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	cargo install $(if $(STACK_SIZES_VER),--version $(STACK_SIZES_VER),) stack-sizes

firsttime_fsp:
	sudo apt-get install build-essential uuid-dev iasl gcc nasm python3-distutils libclang-dev
	git submodule update --init --recursive

debiansysprepare:
	sudo apt-get install device-tree-compiler pkg-config libssl-dev llvm-dev libclang-dev clang qemu-system-x86
	# --default-toolchain is purely an optimization to avoid downloading stable Rust first.
	# -y makes it non-interactive.
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $(TOOLCHAIN_VER)

.PHONY: ciprepare debiansysprepare firsttime
ciprepare: debiansysprepare firsttime firsttime_fsp

update:
	rustup update

# Option used for formatting. If set, the command will only verify if
# formatting is correct (without actually changing the formatting).
# Returns 0 only if all files are properly formatted.
# Usage:
# 	$ make --keep-going format
# 	$ make --keep-going checkformat
check ?=

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
	cd $(dir $@) && make default
.PHONY: testdefault $(TEST_ALL_MAKE_DEFAULT)
testdefault: $(TEST_ALL_MAKE_DEFAULT)

CRATES_TO_FORMAT := $(patsubst %/Makefile,%/Makefile.format,$(ALLMAKEFILE))
$(CRATES_TO_FORMAT):
	cd $(dir $@) && make format -- $(if $(check),--check,)
.PHONY: format $(CRATES_TO_FORMAT)
format: $(CRATES_TO_FORMAT)

CRATES_TO_CHECKFORMAT := $(patsubst %/Makefile,%/Makefile.checkformat,$(ALLMAKEFILE))
$(CRATES_TO_CHECKFORMAT):
	cd $(dir $@) && make checkformat
.PHONY: checkformat $(CRATES_TO_CHECKFORMAT)
checkformat: $(CRATES_TO_CHECKFORMAT)

# There are a number of targets which can not test.
# Once those are fixed, we can just use a test target.
CRATES_TO_TEST := $(patsubst %/Makefile,%/Makefile.clippy,$(ALLMAKEFILE))
$(CRATES_TO_TEST):
	cd $(dir $@) && cargo citest
.PHONY: test $(CRATES_TO_TEST)
test: $(CRATES_TO_TEST)

# TODO: Remove write_with_newline
CRATES_TO_CLIPPY := $(patsubst %/Makefile,%/Makefile.clippy,$(ALLMAKEFILE))
$(CRATES_TO_CLIPPY):
	cd $(dir $@) && make ciclippy
.PHONY: clippy $(CRATES_TO_CLIPPY)
clippy: $(CRATES_TO_CLIPPY)

clean:
	rm -rf $(wildcard src/mainboard/*/*/target)
