use std::path::PathBuf;

use super::Cli;

mod k1x;
mod k1x_hdr;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    K1,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, dir: &PathBuf, features: Vec<String>) {
        match self {
            Board::K1 => k1x::execute_command(command, dir, features),
        }
    }
}
