mod visionfive1;
mod visionfive2;
mod visionfive2_hdr;

use std::path::PathBuf;

use super::Cli;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    VisionFive1,
    VisionFive2,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, directory: &PathBuf, features: Vec<String>) {
        match self {
            Board::VisionFive1 => visionfive1::execute_command(command, directory, features),
            Board::VisionFive2 => visionfive2::execute_command(command, directory, features),
        };
    }
}
