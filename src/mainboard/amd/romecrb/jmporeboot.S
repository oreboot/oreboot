// This works but it's too early in the game,
// protected start wants to load seg registers and we don't
// want that.
// push $0x76ff0097
// 76ff00b7 <bootblock_protected_mode_entry>:
//movl $0x10000, %esp
//push $0x76ff0088
//push $0x76ff0098
push $0x76ff0000

ret
// 724  as -o jmporeboot jmporeboot.S 
//  726  objcopy -O binary jmporeboot jmporeboot.bin
