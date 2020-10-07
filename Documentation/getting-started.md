# Getting Started

If you would like to contribute, but don't know how to start, this page is for
you. This guide will walk you through running oreboot locally using QEMU
emulation and explain the high level architecture / structure of the project.
It also assumes the use of a Debian like system.

## Set up the environment

The first step, is to install all dependencies and ensure the project can be
started locally (see `README.md` and `azure-pipelines.yml` for possibly more
up-to-date instructions.

```shell
git clone git://github.com/oreboot/oreboot
cd oreboot
make debiansysprepare
make firsttime
```

## Device Tree

Read the module comment inside `src/lib/device_tree/src/lib.rs`. In oreboot,
device tree is used for two reasons - the actual device tree of the hardware
that we are running on (we will refer to it as `hardware device tree`) and
device tree used to define the layout of the image that is flashed into the device
(we will refer to it as `oreboot device tree`.

## What `make run` actually does on a mainboard

1. Build oreboot specific for the given mainboard.
2. Create an binary image that will be flashed into the device.
3. Run given image using QEMU emulation.

Let's go over above steps in a little more details.

### Build oreboot (bootblob.bin)

This is the code inside `src/mainboard/*/*/src/main.rs`. It starts with a tiny
bit of assembler (that is included inside `main.rs`, it performs minimal
amount of initialization and then calls into the Rust program. Compared to
other Rust binaries, there is no `main` method, instead you will see `pub
extern "C" fn ...` methods that are being called from assembler to start the
program. The Rust binary then initializes more of the hardware, prepares the
actual payload and runs it.

Mainboard code only uses `core` library and does not use `std`.  This means no
heap allocated structures like `Box` or `Vec` and arrays have to have
statically allocated size.

The binary is generated in two steps:

* Build ELF (Executable and Linkable Format) binary, with `cargo xbuild` command.
* Convert it to binary format with `rust-objcopy` into `bootblob.bin` file.

This `bootblob.bin` is then used to construct the image that will be written
into the device.

### Create image (image.bin)

After we got the oreboot binary, we need to construct an image that will be
flashed into the device. `layoutflash` (whose source code is located inside
`tools/layoutflash` directory) takes `oreboot device tree` specification of
image layout, and constructs a binary image. It basically just concatenates
multiple binary blobs together at the right positions.

Each mainboard has file called `fixed-dtfs.dts` which is human readable
specification of the image layout (all tools use the binary form of this file
which is converted inside Makefile using `dtc` command).

Most images include following parts:

* oreboot (`bootblob.bin`) - this is what hardware starts first. It often is a
  first thing in the image (though it not always starts at offset 0).
* `oreboot device tree` - binary form of `fixed-dtfs.dts` file. I believe
  currently it is included mostly for debugging purposes (though my guess is
  that it should be used to find where the payload is located that should be
  executed, but currently the offsets are hardcoded inside the oreboot binary).
* payload - actual binary that will be executed. It is configured with
  environment variables, which is why the `make run` method starts with
  `PAYLOAD_A=...`.

### Run QEMU

Once image is created, qemu is invoked to emulate given hardware. It takes
`image.bin` produced in previous step and writes it on to the device.

## Important crates / types

* `src/drivers/model/` - defines the `Device` trait, that is the main
  type to read and write data (often to actual serial devices)
* `src/drivers/wrappers/` - defines few simple implementation of
  `Device` tree (e.g. `SliceReader` for exposing slice of binary data as
  `Driver` trait, or `Memory` for reading the data directly from memory).
* `src/lib/device_tree/` - library for reading device tree binary format.
* `payloads` - library for loading a payload and executing it.
* `tools/layoutflash` - tool for creating an image from binary blobs.

Note: There are probably more important types and crates, but those are the
ones I got to so far in my exploration.

## Important make commands

Those commands are invoked from the top level `oreboot` directory:

* `make format` - format all Rust files in the project.
* `make clippy` - run clippy linter on all Rust files in the project.
* `make test` - run all Rust tests in the project.
* `make flash` - flash to hardware, specify programmer for flashrom with
  `FLASHROM_PROGRAMMER`.


## Next steps

* I highly recommend starting with either a tool like `layoutflash` or a
  mainboards `main.rs` file, and reading its code. When something is not clear,
  try to improve the comments, write unit tests or refactor.
* Look at open Issues in github, especially the ones with `good first issue`
  label.
