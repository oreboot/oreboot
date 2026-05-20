## Toolchain

`rustc --print target-spec-json --target armv7a-none-eabi -Z unstable-options`

```
{
  "abi": "eabi",
  "arch": "arm",
  "c-enum-min-bits": 8,
  "crt-objects-fallback": "false",
  "data-layout": "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
  "disable-redzone": true,
  "emit-debug-gdb-scripts": false,
  "features": "+v7,+thumb2,+soft-float,-neon,+strict-align",
  "linker": "rust-lld",
  "linker-flavor": "gnu-lld",
  "llvm-floatabi": "soft",
  "llvm-target": "armv7a-none-eabi",
  "max-atomic-width": 64,
  "metadata": {
    "description": "Bare Armv7-A",
    "host_tools": false,
    "std": false,
    "tier": 2
  },
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-pointer-width": 32
}
```
