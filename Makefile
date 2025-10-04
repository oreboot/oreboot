help:
	@echo 'Make options:'
	@echo 'firsttime -- for the first time you run make'
	@echo 'update -- to update the install'
	@echo 'format -- to format all files'

# NOTE: These are the host utilities, requiring their own recent Rust version.
RUST_VER := 1.90
BINUTILS_VER := 0.4.0
DPRINT_VER := 0.50.2

# cargo command wrapper and shorthands
CARGO := rustup run --install $(RUST_VER) cargo
CARGOINST := $(CARGO) install
CARGOTEST := $(CARGO) test --release

.PHONY: firsttime
firsttime:
	$(CARGOINST) $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	$(CARGOINST) $(if $(DPRINT_VER),--version $(DPRINT_VER),) dprint

.PHONY: nexttime
nexttime:
	$(CARGOINST) --force $(if $(BINUTILS_VER),--version $(BINUTILS_VER),) cargo-binutils
	$(CARGOINST) --force $(if $(DPRINT_VER),--version $(DPRINT_VER),) dprint

.PHONY: debiansysprepare
debiansysprepare: rustprepare
	# -y makes the install command non-interactive.
	sudo apt-get install -y device-tree-compiler qemu-system

.PHONY: rustprepare
rustprepare:
	# -y makes the installation non-interactive.
	curl https://sh.rustup.rs -sSf | sh -s -- -y

.PHONY: update
update:
	rustup update
	rustup toolchain install

.PHONY: ciprepare
ciprepare: debiansysprepare firsttime update

.PHONY: format
format:
	dprint fmt

.PHONY: checkformat
checkformat:
	dprint check

clippy:
	$(CARGO) clippy -- -D warnings

MAINBOARDS := $(wildcard src/mainboard/*/*/Makefile)

.PHONY: $(MAINBOARDS)
mainboards: $(MAINBOARDS)
$(MAINBOARDS):
	make --no-print-directory -C $(dir $@)

.PHONY: test
test:
	$(CARGOTEST) --package layoutflash

# convenience target: this should be the full ci flow
checkandbuildall: ciprepare clippy checkformat mainboards
	echo "Done CI!"

clean:
	rm -rf $(TOP)/target
