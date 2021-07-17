# Rust Preprocessor

rpp is a simple-as-possible preprocessor implementation in Rust. It is designed
for the primary purpose of preprocessing oreboot assembly files to allow for
code reuse and macro definitions.

rpp uses the same syntax as GNU cpp and supports the following preprocessor
directives:

* `#define`
* `#include`

Note, rpp does not support function macro definitions. For example:

A simple macro definition is supported:

```
#define NAME 0xFFFF // Supported
```

but the following function is not:

```
#define FUNC(x) x##_SOMETHING
```
