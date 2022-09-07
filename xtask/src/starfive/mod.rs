mod visionfive1;
use super::Cli;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    VisionFive1,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, features: Vec<String>) {
        match self {
            Board::VisionFive1 => visionfive1::execute_command(command, features),
        };
    }
}
