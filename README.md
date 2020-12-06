oreboot README
==============

[![Build Status](https://dev.azure.com/azure0427/Oreboot%20Pipeline/_apis/build/status/oreboot.oreboot?branchName=master)](https://dev.azure.com/azure0427/Oreboot%20Pipeline/_build/latest?definitionId=1&branchName=master)

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

Oreboot+QEMU for aarch64:

[![asciicast](https://asciinema.org/a/lbvalLX1JlAwL6gsYC2FYM2jh.svg)](https://asciinema.org/a/lbvalLX1JlAwL6gsYC2FYM2jh)


Getting oreboot
---------------

Clone this repo and enter its directory, i.e.:

```sh
git clone git://github.com/oreboot/oreboot
cd oreboot
```

Prerequisites
-------------

In general, you will need the following packages installed:

- `device-tree-compiler`
- `pkg-config`
- `libssl`
- `rustup`

For Debian based systems, there is a make target to install those, which pulls
`rustup` through curl from https://sh.rustup.rs:

```sh
make debiansysprepare
```

Otherwise, install the package through your system package manager.

Setting up the toolchain
------------------------

Regardless of your OS, you will need to install the toolchain for oreboot.
This command only needs to be done once but it is safe to do it repeatedly.

```sh
make firsttime
```


Keeping build tools up to date
------------------------------

Each time you start to work with oreboot, or even daily:

```
cd oreboot
make update
```

You should definitely do this before reporting any issues.


Building oreboot
----------------

To build oreboot for a specific platform, do this:

```
# Go to the mainboard's directory.
cd src/mainboard/sifive/hifive
# Build in release mode.
make
# Build in debug mode.
MODE=debug make
# View disassembly
make objdump
# Run in QEMU simulation.
make run
# Flash with flashrom.
make flash
```

The root Makefile allows you to quickly build all platforms:

```
# build all mainboards
make mainboards
# build everything in parallel
make -j mainboards
```


QEMU
----

```
# Install QEMU for your target platform, e.g. x86
sudo apt install qemu-system-x86

# Build release build and start with QEMU
cd src/mainboard/emulation/qemu-q35 && make run
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

To build QEMU from source for aarch64:

```
git clone https://github.com/qemu/qemu && cd qemu
mkdir build-aarch64 && cd build-aarch64
../configure --target-list=aarch64-softmmu
make -j$(nproc)
# QEMU binary is at aarch64-softmmu/qemu-system-aarch64


Oreboot Mainboards
------------------

* Emulation
  * qemu-armv7
  * qemu-aarch64
  * qemu-q35
  * qemu-riscv
* Hardware
  * Aspeed ast25x0
  * Nuvoton npcm7xx
  * OpenTitan crb, [Documentation](Documentation/opentitan/README.md)
  * SiFive HiFive Unleashed, [Documentation](Documentation/sifive/setup.md)


Ground Rules
------------

* Makefile must be simple. They cannot contain control flow.
* Cargo.toml files are located in the src/mainboard/x/y directories. which will
  allow us to build all boards in parallel.
* All code is auto-formatted with rustfmt with no exceptions. There are no
  vestiges of the 19th century such as line length limits.
* There will be no C.
* We will not run our own Gerrit. We are using Github for now, and the github
  Pull Request review mechanism.
* We will not run our own Jenkins. We will use the most appropriate CI; for
  now, that is Azure but we will be flexible.


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
