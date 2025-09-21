use super::Cli;

mod egon;
mod h616;
mod nezha;
mod xfel;

#[derive(Debug, Clone, Copy)]
pub enum Board {
    H616,
    Nezha,
}

impl Board {
    pub(crate) fn execute_command(self, command: &Cli, features: Vec<String>) {
        match self {
            Board::H616 => h616::execute_command(command, features),
            Board::Nezha => nezha::execute_command(command, features),
        };
    }
}
