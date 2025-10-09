use std::path::PathBuf;

use super::Cli;

mod nezha;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    Nezha,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, dir: &PathBuf, features: Vec<String>) {
        match self {
            Board::Nezha => nezha::execute_command(command, dir, features),
        };
    }
}
