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

Oreboot+QEMU for ARM:

[![asciinema](https://asciinema.org/a/Ne4Fwa4Wpt95dorEoVnHwiEkP.png)](https://asciinema.org/a/Ne4Fwa4Wpt95dorEoVnHwiEkP)


Build Requirements
------------------

 * Rust
 * Device tree compiler


Building oreboot
-----------------

To setup your Rust environment for oreboot, see below:

```
# Install some tools we will need later
sudo apt install device-tree-compiler pkg-config libssl-dev

# Install rustup
curl https://sh.rustup.rs -sSf | sh

# Apply the Rust environment to your current shell
source $HOME/.cargo/env

# Install cargo-make
cargo install cargo-make

# NOTE: The following needs to be run in the root of the oreboot directory
cd oreboot/
rustup override set nightly   # Set the nightly rust compiler to be used for oreboot
cargo make setup              # Install a few compiler tools
```

Occasionally you want to run `rustup update` to keep your Rust up-to-date.
A good idea is to make sure you have the latest version before reporting any
issues.

To build oreboot for a specific platform, do like this:

```
# Build for RISC-V
export OREBOOT="${PWD}"
cd src/mainboard/sifive/hifive
cargo make              # Debug
cargo make -p release   # Optimized

# View disassembly
cargo make objdump -p release

# Run in QEMU simulation
cargo make run -p release

# Alternatively, without setting OREBOOT, you can do like this
cargo make --env OREBOOT="${PWD}" --cwd src/mainboard/sifive/hifive
```

QEMU
----

```
# Install QEMU for your target platform, e.g. x86
sudo apt install qemu-system-x86

# Build release build and start with QEMU
cargo make --env OREBOOT="${PWD}" --cwd src/mainboard/emulation/qemu-q35 run -p release
# Quit qemu with CTRL-A X
```

To build QEMU from source for RISC-V:

```
git clone https://github.com/qemu/qemu && cd qemu
mkdir build-riscv64 && cd build-riscv64
../configure --target-list=riscv64-softmmu
make -j$(nproc)
# QEMU binary is at riscv64-softmmu/qemu-system-riscv64
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

Ground Rules for x86
--------------------

* We prefer all pieces of the firmware to be open-source; but can accept an ME
  and FSP binary blob for x86 architectures.
* Blobs must be essential to boot the system and not provide any extraneous
  functionality which could not be implemented in Oreboot.
* Blobs must be redistributable and are ideally available on GitHub.
* Blobs must not be submitted to github.com/oreboot/oreboot. We prefer blobs to
  be submitted to github.com/oreboot/blobs, github.com/coreboot/blobs or some
other GitHub repository.
* The blobs must be in a binary format. No additional C code, assembly files or
  header files are acceptable.
* Any compromises to the language safety features of Rust must be explicitly
  stated.

As a "measure" for how open-source firmware is, use the percentage of the final
binary size. For example, if 70% of the firmware bytes are closed-source blob
and 30% built from Oreboot source code, we would say the firmware is 30%
open-source.


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
