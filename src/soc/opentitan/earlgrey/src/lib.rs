#![no_std]
#![feature(global_asm)]

global_asm!(include_str!("bootblock.S"));
