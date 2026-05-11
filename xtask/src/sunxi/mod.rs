use std::path::PathBuf;

use super::Cli;

mod egon;
mod fel;
mod h616;
mod nezha;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    H616,
    Nezha,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, dir: &PathBuf, features: Vec<String>) {
        match self {
            Board::H616 => h616::execute_command(command, dir, features),
            Board::Nezha => nezha::execute_command(command, dir, features),
        };
    }
}
