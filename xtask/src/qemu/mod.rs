use std::path::PathBuf;

use super::Cli;

mod riscv;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    RiscV,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, dir: &PathBuf, features: Vec<String>) {
        match self {
            Board::RiscV => riscv::execute_command(command, dir, features),
        }
    }
}
