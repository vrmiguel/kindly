use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No command was supplied to be executed")]
    NoCommandToRun,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
