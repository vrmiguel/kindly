use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No command was supplied to be executed")]
    NoCommandToRun,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to query password bank")]
    PasswordBank,
    #[error("Failed to query shadow file")]
    ShadowFile,
    #[error("Interior nul byte error")]
    UnixString,
    #[error("Authentication failed")]
    Authentication
}

pub type Result<T> = std::result::Result<T, Error>;
