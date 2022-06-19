mod nezha;
use super::Cli;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    Nezha,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, features: Vec<String>) {
        match self {
            Board::Nezha => nezha::execute_command(command, features),
        };
    }
}
