oreboot README
===============

oreboot is a downstream fork of coreboot, i.e. oreboot is coreboot without 'c'.

oreboot will only target truly open systems requiring no binary blobs. For now, that means no x86.
oreboot is mostly written in Rust, with assembly where needed.

oreboot currently only plans to support LinuxBoot payloads.


Demo
----

Oreboot+QEMU for RISC-V HiFive Unleased:

[![asciicast](https://asciinema.org/a/XnWkMWTABuajsbGPMMTefjuZ2.svg)](https://asciinema.org/a/XnWkMWTABuajsbGPMMTefjuZ2)

Oreboot+QEMU for AST2500:

[![asciinema](https://asciinema.org/a/Ne4Fwa4Wpt95dorEoVnHwiEkP.png)](https://asciinema.org/a/Ne4Fwa4Wpt95dorEoVnHwiEkP)


Build Requirements
------------------

 * Rust
 * Device tree compiler


Building oreboot
-----------------

```
# Install rustup
curl https://sh.rustup.rs -sSf | sh

# Run this in the oreboot project directory. This uses the nightly rust
# compiler for the oreboot directory.
rustup override set nightly

# Install cargo-make
cargo install cargo-make

# Ocassionally run:
rustup update

# Install a few compiler tools.
cargo make setup
sudo apt-get install device-tree-compiler

# Build for RISC-V
cd src/mainboard/sifive/hifive
cargo make              # Debug
cargo make -p release   # Optimized

# View disassembly
cargo make objdump -p release
```

QEMU
----

```
sudo apt-get install qemu-system-arm
cargo make run -p release

# Quit qemu with CTRL-A X
```

To build QEMU from source for riscv:

```
git clone https://github.com/qemu/qemu && cd qemu
mkdir build-riscv64 && cd build-riscv64
../configure --help
../configure --target-list=riscv64-softmmu
make -j16
stat riscv64-softmmu/qemu-system-riscv64
```

Oreboot on Hardware
-------------------

* [HiFive Unleashed](Documentation/sifive/setup.md)


Website and Mailing List
------------------------

Not yet.

Ground Rules
------------------------

* The build tool is cargo-make; there will be no GNU Makefiles.
* Cargo.toml files are located in the src/mainboard/x/y directories. which will allow us to build all boards in parallel.
* All code is auto-formatted with rustfmt with no exceptions. There are no vestiges of the 19th century such as line length limits.
* There will be no C.
* We will not run our own Gerrit. We are using Github for now, and the github Pull Request review mechanism.
* We will not run our own Jenkins. We will use the most appropriate CI; for now, that is Azure but we will be flexible.

Copyright and License
---------------------

The copyright on oreboot is owned by quite a large number of individual
developers and companies. Please check the individual source files for details.

oreboot is licensed under the terms of the GNU General Public License (GPL).
Some files are licensed under the "GPL (version 2, or any later version)",
and some files are licensed under the "GPL, version 2". For some parts, which
were derived from other projects, other (GPL-compatible) licenses may apply.
Please check the individual source files for details.

This makes the resulting oreboot images licensed under the GPL, version 2.
