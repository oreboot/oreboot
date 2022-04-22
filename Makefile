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
BINUTILS_VER := 0.3.4
STACK_SIZES_VER := 0.4.0
TARPAULIN_VER := 0.19.1

CARGOINST := rustup run --install nightly cargo install

.PHONY: $(MAINBOARDS)
mainboards: $(MAINBOARDS)
$(MAINBOARDS):
	cd $(dir $@) && make cibuild

firsttime:
	$(CARGOINST) $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	$(CARGOINST) $(if $(STACK_SIZES_VER),--version $(STACK_SIZES_VER),) stack-sizes
	$(CARGOINST) $(if $(TARPAULIN_VER),--version $(TARPAULIN_VER),) cargo-tarpaulin
	rustup target add riscv64imac-unknown-none-elf
	rustup target add riscv64gc-unknown-none-elf
	rustup target add aarch64-unknown-none-softfloat
	rustup target add riscv32imc-unknown-none-elf

nexttime:
	rustup toolchain install $(TOOLCHAIN_VER)
	rustup component add llvm-tools-preview rust-src --toolchain $(TOOLCHAIN_VER)
	$(CARGOINST) --force $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	$(CARGOINST) --force $(if $(STACK_SIZES_VER),--version $(STACK_SIZES_VER),) stack-sizes
	$(CARGOINST) --force $(if $(TARPAULIN_VER),--version $(TARPAULIN_VER),) cargo-tarpaulin
	rustup target add riscv64imac-unknown-none-elf
	rustup target add riscv64gc-unknown-none-elf

debiansysprepare:
	sudo apt-get install device-tree-compiler pkg-config libssl-dev llvm-dev libclang-dev clang qemu-system-x86
	# --default-toolchain is purely an optimization to avoid downloading stable Rust first.
	# -y makes it non-interactive.
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $(TOOLCHAIN_VER)

.PHONY: ciprepare debiansysprepare firsttime
ciprepare: debiansysprepare firsttime

update:
	rustup update

# Option used for formatting. If set, the command will only verify if
# formatting is correct (without actually changing the formatting).
# Returns 0 only if all files are properly formatted.
# Usage:
# 	$ make --keep-going format
# 	$ make --keep-going checkformat
check ?=

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
CRATES_TO_TEST := $(patsubst %/Makefile,%/Makefile.test,$(ALLMAKEFILE))
$(CRATES_TO_TEST):
	cd $(dir $@) && make test
.PHONY: test $(CRATES_TO_TEST)

# NOTE: In CI, we run tests with coverage report.
# The individual crates' Makefiles use the `citest` target.
# However, there are a number of crates which can not test.
# Hence, `citest` either points to `coverage` or `skiptest`.
# We use the LCOV format so that we can simply concatenate
# the multiple reports. See ./Makefile.inc for details.
CRATES_TO_CITEST := $(patsubst %/Makefile,%/Makefile.citest,$(ALLMAKEFILE))
$(CRATES_TO_CITEST):
	cd $(dir $@) && make citest
.PHONY: test $(CRATES_TO_CITEST)
citest: $(CRATES_TO_CITEST)
	# concatenate all the results from the indidividual directories
	mkdir -p coverall
	find . -name "lcov.info" -exec cat > coverall/lcov.txt {} +


# TODO: Remove write_with_newline
CRATES_TO_CLIPPY := $(patsubst %/Makefile,%/Makefile.clippy,$(ALLMAKEFILE))
$(CRATES_TO_CLIPPY):
	cd $(dir $@) && make ciclippy
.PHONY: clippy $(CRATES_TO_CLIPPY)
clippy: $(CRATES_TO_CLIPPY)

# convenience target: this should be the full ci flow

checkandbuildall: ciprepare clippy checkformat test mainboards
	echo "Done CI!"

clean:
	rm -rf $(TOP)/target
