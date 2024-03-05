mod aarch64;
mod riscv;

use super::Cli;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    RiscV,
    AArch64,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, features: Vec<String>) {
        match self {
            Board::RiscV => riscv::execute_command(command, features),
            Board::AArch64 => aarch64::execute_command(command, features),
        }
    }
}
