# QEMU FSP

This target uses the QEMU x64 version of FSP found in the FSP SDK:
https://github.com/universalpayload/fspsdk/tree/qemu_fsp_x64

This is not generally useful seeing that QEMU has no hardware for FSP to
initialize. It is intended to test the bindings between Rust and FSP.

## Getting Started

1. Do everything in the main README.md file.
2. Run `make firsttime_fsp`
3. `cd src/mainboard/emulation/qemu-fsp`
4. `make run`

You should see FSP-M and FSP-S run in QEMU:

```
Calling FspMemoryInit at 0xfff80000+0x16cde

============= FSP Spec v2.0 Header Revision v3 ($QEMFSP$ v0.0.10.10) =============
Fsp BootFirmwareVolumeBase - 0xFFF95000
Fsp BootFirmwareVolumeSize - 0x22000
Fsp TemporaryRamBase       - 0x20000000
Fsp TemporaryRamSize       - 0x10000
Fsp PeiTemporaryRamBase    - 0x20000000
Fsp PeiTemporaryRamSize    - 0xA666
Fsp StackBase              - 0x2000A666
Fsp StackSize              - 0x599A
Register PPI Notify: DCD0BE23-9586-40F4-B643-06522CED4EDE
Install PPI: 8C8CE578-8A3D-4F1C-9935-896185C32DD3
Install PPI: 5473C07A-3DCB-4DCA-BD6F-1E9689E7349A
The 0th FV start address is 0x000FFF95000, size is 0x00022000, handle is 0xFFF95000
Register PPI Notify: 49EDB1C1-BF21-4761-BB12-EB0031AABB39
Register PPI Notify: EA7CA24B-DED5-4DAD-A389-BF827E8F9B38
Install PPI: B9E0ABFE-5979-4914-977F-6DEE78C278A6
Install PPI: A1EEAB87-C859-479D-89B5-1461F4061A3E
Install PPI: DBE23AA9-A345-4B97-85B6-B226F1617389
DiscoverPeimsAndOrderWithApriori(): Found 0x2 PEI FFS files in the 0th FV
Loading PEIM 9B3ADA4F-AE56-4C24-8DEA-F03B7558AE50
Loading PEIM at 0x000FFFA4468 EntryPoint=0x000FFFA6C8F PcdPeim.efi
Install PPI: 06E81C58-4AD7-44BC-8390-F10265F72480
Install PPI: 01F34D25-4DE2-23AD-3FF3-36353FF323F1
Install PPI: 4D8B155B-C059-4C8F-8926-06FD4331DB8A
Install PPI: A60C6B59-E459-425D-9C69-0BCC9CB27D81
Register PPI Notify: 605EA650-C65C-42E1-BA80-91A52AB618C6
Loading PEIM 9E1CC850-6731-4848-8752-6673C7005EEE
Loading PEIM at 0x000FFFA72A4 EntryPoint=0x000FFFA958B FspmInit.efi
FspmInitPoint() - Begin
BootMode : 0x0
Install PPI: 7408D748-FC8C-4EE6-9288-C4BEC092A410
Register PPI Notify: F894643D-C449-42D1-8EA8-85BDD8C65BDE
PeiInstallPeiMemory MemoryBegin 0x7EF00000, MemoryLength 0x100000
FspmInitPoint() - End
Temp Stack : BaseAddress=0x2000A666 Length=0x599A
Temp Heap  : BaseAddress=0x20000000 Length=0xA666
Total temporary memory:    65536 bytes.
  temporary memory stack ever used:       22938 bytes.
  temporary memory heap used for HobList: 3008 bytes.
  temporary memory heap occupied by memory pages: 0 bytes.
Old Stack size 22938, New stack size 131072
Stack Hob: BaseAddress=0x7EF00000 Length=0x20000
Heap Offset = 0x5EF20000 Stack Offset = 0x5EF10000
Loading PEIM 52C05B14-0B98-496C-BC3B-04B50211D680
Loading PEIM at 0x0007EFF3160 EntryPoint=0x0007EFFBCA4 PeiCore.efi
Reinstall PPI: 8C8CE578-8A3D-4F1C-9935-896185C32DD3
Reinstall PPI: 5473C07A-3DCB-4DCA-BD6F-1E9689E7349A
Reinstall PPI: B9E0ABFE-5979-4914-977F-6DEE78C278A6
Install PPI: F894643D-C449-42D1-8EA8-85BDD8C65BDE
Notify: PPI Guid: F894643D-C449-42D1-8EA8-85BDD8C65BDE, Peim notify entry point: FFFA8DC5
Memory Discovered Notify invoked ...
FSP TOLM = 0x7F000000
Migrate FSP-M UPD from 1DE8 to 7EFF2000 
FspMemoryInitApi() - [Status: 0x00000000] - End
Returned 0
Calling FspSiliconInit at 0xfff80000+0x494
FspSiliconInitApi() - Begin
Install PPI: 49EDB1C1-BF21-4761-BB12-EB0031AABB39
Notify: PPI Guid: 49EDB1C1-BF21-4761-BB12-EB0031AABB39, Peim notify entry point: FFF9D4CF
The 1th FV start address is 0x000FFF80000, size is 0x00015000, handle is 0xFFF80000
DiscoverPeimsAndOrderWithApriori(): Found 0x4 PEI FFS files in the 1th FV
Loading PEIM 86D70125-BAA3-4296-A62F-602BEBBB9081
Loading PEIM at 0x0007EFEC160 EntryPoint=0x0007EFEFC9A DxeIpl.efi
Install PPI: 1A36E4E7-FAB6-476A-8E75-695A0576FDD7
Install PPI: 0AE8CE5D-E448-4437-A8D7-EBF5F194F731
Loading PEIM 131B73AC-C033-4DE1-8794-6DAB08E731CF
Loading PEIM at 0x0007EFE2000 EntryPoint=0x0007EFE32DA FspsInit.efi
FspInitEntryPoint() - start
Register PPI Notify: 605EA650-C65C-42E1-BA80-91A52AB618C6
Register PPI Notify: BD44F629-EAE7-4198-87F1-39FAB0FD717E
Register PPI Notify: 7CE88FB3-4BD7-4679-87A8-A8D8DEE50D2B
Register PPI Notify: 6ECD1463-4A4A-461B-AF5F-5A33E3B2162B
Register PPI Notify: 30CFE3E7-3DE1-4586-BE20-DEABA1B3B793
FspInitEntryPoint() - end
Loading PEIM BA37F2C5-B0F3-4A95-B55F-F25F4F6F8452
Loading PEIM at 0x0007EFD6000 EntryPoint=0x0007EFD7E8D QemuVideo.efi
NO valid graphics config data found!
Loading PEIM 29CBB005-C972-49F3-960F-292E2202CECD
Loading PEIM at 0x0007EFCC000 EntryPoint=0x0007EFCD555 FspNotifyPhasePeim.efi
The entry of FspNotificationPeim
Reinstall PPI: 0AE8CE5D-E448-4437-A8D7-EBF5F194F731
DXE IPL Entry
FSP HOB is located at 0x7EF20000
Install PPI: 605EA650-C65C-42E1-BA80-91A52AB618C6
Notify: PPI Guid: 605EA650-C65C-42E1-BA80-91A52AB618C6, Peim notify entry point: FFFA6B09
Notify: PPI Guid: 605EA650-C65C-42E1-BA80-91A52AB618C6, Peim notify entry point: 7EFE314C
FspInitEndOfPeiCallback++
FspInitEndOfPeiCallback--
FSP is waiting for NOTIFY
FspSiliconInitApi() - [Status: 0x00000000] - End
Returned 0
```

## Build System

1. The source code for fspsdk is in a git submodule under 3rdparty.
2. The build script at
   [src/vendorcode/fsp/qemu/build.rs](https://github.com/oreboot/oreboot/blob/main/src/vendorcode/fsp/qemu/build.rs)
   runs the `BuildFsp.py` script to build the QEMU FSP. 
3. The structs and constants from the fspsdk C header files are transformed to
   Rust code at build time using Rust's
   [bindgen](https://rust-lang.github.io/rust-bindgen/) crate. Bindgen uses
   clang to parse the C code, so the translation is exceedingly accurate. The
   generated structs are available in the
   [src/vendorcode/fsp/qemu](https://github.com/oreboot/oreboot/blob/main/src/vendorcode/fsp/qemu/build.rs)
   crate.
4. The QEMUFSP.fd binary contains 3 firmware volumes for FSP-S, FSP-M and
   FSP-T. It is included into the Rust compilation here:
   https://github.com/oreboot/oreboot/blob/main/src/vendorcode/fsp/qemu/src/lib.rs#L18
5. The linker script places the fspblob at a fixed address 0xfff80000 here:
   https://github.com/oreboot/oreboot/blob/main/src/mainboard/emulation/qemu-fsp/link.ld#L14
   padded by 0xff
