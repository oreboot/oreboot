mod riscv;

use super::Cli;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    RiscV,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, features: Vec<String>) {
        match self {
            Board::RiscV => riscv::execute_command(command, features),
        }
    }
}
