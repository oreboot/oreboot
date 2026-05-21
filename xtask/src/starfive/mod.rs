use std::path::PathBuf;

use super::Cli;

mod visionfive1;
mod visionfive2;
mod visionfive2_hdr;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    VisionFive1,
    VisionFive2,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, dir: &PathBuf, features: Vec<String>) {
        match self {
            Board::VisionFive1 => visionfive1::execute_command(command, dir, features),
            Board::VisionFive2 => visionfive2::execute_command(command, dir, features),
        };
    }
}
