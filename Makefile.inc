# This file is included by all Makefiles. The user is required to set
# the path to OREBOOT.
TOP         = $(OREBOOT)
MODE       ?= release
TOOLS_DIR   = $(TOP)/tools
# Cargo will build the tools using the target from the .cargo directory which
# does not work so well if you are cross-compiling. This shell one-liner gets
# the target of the current system which can be passed via --target for
# building tools
TOOLS_TARGET = $(shell rustc -vV | sed -n -E 's/host: (.+)/\1/p')
CARGO_FLAGS = --verbose

# Override variables depending on the build.
include $(TOP)/Makefile.mode.$(MODE).inc

$(shell echo "***** Oreboot Build Config *****" 1>&2)
$(shell echo "TOP          = $(TOP)" 1>&2)
$(shell echo "MODE         = $(MODE)" 1>&2)
$(shell echo "TARGET       = $(TARGET)" 1>&2)
$(shell echo "TARGET_DIR   = $(TARGET_DIR)" 1>&2)
$(shell echo "TOOLS_TARGET = $(TOOLS_TARGET)" 1>&2)
$(shell echo "cargo --version  = $(shell cargo --version)" 1>&2)
$(shell echo "rustc --version  = $(shell rustc --version)" 1>&2)
$(shell echo "rustup --version = $(shell rustup --version)" 1>&2)
$(shell echo "**********" 1>&2)

# The default target.
# There can be several of these; a mainboard might specify
# "build something" with a default:: target
default::

# format things
format:
	cargo fmt

checkformat:
	cargo fmt -- --check

clean:
	rm -rf target/

# Clippy is the standard rule
clippy:
	cargo clippy -- -D warnings -A clippy::write_with_newline

fix:
	cargo clippy --fix -- -D warnings -A clippy::write_with_newline

# This is a temporary bandaid until clippy is fixed, for the
# few boards that need it. To figure out which ones use it,
# just git grep skipclippy
skipclippy:
	echo Not doing clippy for this target

# Testing. A lot of directories can't test either, yet.
test:
	cargo tarpaulin --ciserver travis-ci --coveralls ABC ;
	cargo tarpaulin --out Xml;
	wget -q "https://codecov.io/bash" && chmod u+x ./bash && ./bash ;

skiptest:
	echo Not doing test for this target
