use aarch64_cpu::asm;

fn boot_to_kernel() -> ! {
    asm::eret()
}

pub fn romstage() -> ! {
    boot_to_kernel()
}
