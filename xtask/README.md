# oreboot build system

We use [xtask](https://github.com/matklad/cargo-xtask/) for additional steps
besides compiling the actual code in oreboot. That may contain adding vendor
specific headers, possibly with checksums, and integrating with other tools.
