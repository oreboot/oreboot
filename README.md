oreboot README
==============

[![Build Status](
https://dev.azure.com/azure0427/Oreboot%20Pipeline/_apis/build/status/oreboot.oreboot?branchName=main)](
https://dev.azure.com/azure0427/Oreboot%20Pipeline/_build/latest?definitionId=1&branchName=main)

![oreboot logo](Documentation/img/logo-small.png)

oreboot is a downstream fork of coreboot, i.e. oreboot is coreboot without 'c'.

oreboot is mostly written in Rust, with assembly where needed.

oreboot currently only plans to support LinuxBoot payloads.


Demos
-----

- [oreboot for ARM in QEMU](https://asciinema.org/a/Ne4Fwa4Wpt95dorEoVnHwiEkP)
- [oreboot for RISC-V HiFive Unleashed in QEMU](https://asciinema.org/a/XnWkMWTABuajsbGPMMTefjuZ2)

### Output sample from the RISC-V demo
```
Testing DDR...                                                                                              
Starting to fill with data                                                                                  
Starting to read back data                                                                                  
Passed                                                                                                      
Loading payload                                                                                             
Running payload                                                                                             
ACEGHIJLMN                                                                                                  
Linux version 5.3.0-rc5+ (orebootorebootorebootorebooto) (gcc version 8.2.0 (Debian 8.2.0-14+build1)) #9 Tue
 Sep 3 06:42:07 PDT 2019                                                                                    
Zone ranges:                                                                                                
  DMA32    [mem 0x0000000080000000-0x00000000bfffffff]                                                      
  Normal   empty                                                                                            
Movable zone start for each node                                                                            
Early memory node ranges                                                                                    
  node   0: [mem 0x0000000080000000-0x00000000bfffffff]                                                     
Initmem setup node 0 [mem 0x0000000080000000-0x00000000bfffffff]                                            
elf_hwcap is 0x112d                                                                                         
Built 1 zonelists, mobility grouping on.  Total pages: 258560                                               
Kernel command line: console=ttySIF0                                                                        
Dentry cache hash table entries: 131072 (order: 8, 1048576 bytes, linear)                                   
Inode-cache hash table entries: 65536 (order: 7, 524288 bytes, linear)                                      
mem auto-init: stack:off, heap alloc:off, heap free:off                                                     
Memory: 1031428K/1048576K available (814K kernel code, 80K rwdata, 98K rodata, 64K init, 171K bss, 17148K re
served, 0K cma-reserved) 
```

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

Developing oreboot
------------------

There are two different things in the project:

1. `src/mainboards/*` the actual targets; those depend on and shared crates, which
   can be drivers, SoC init code, and similar. For mainboards, `Cargo.lock`
   **must** be tracked.
2. `src/*` everything else; these are the aforementioned crates, for which, we
    do not track the `Cargo.lock` files.

Checking in a mainboard's `Cargo.lock` file records the state of its dependencies
at the time of a successful build, enabling reproducibility. Ideally, a lock file
is updated follwoing successful boot on hardware.

For more, see: https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries

When creating a new mainboard, looking at how others are set up for the same
architecture is a good start. Be aware that oreboot is targeting bare metal, so
there is no standard library available.

Building oreboot
----------------

If the mainboard uses FSP (Intel platforms), download the blobs with:

```
git submodule update --init
```

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
```

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
