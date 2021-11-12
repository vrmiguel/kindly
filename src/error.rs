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
    Authentication,
    #[error("Failed to set UID")]
    Setuid,
    #[error("Some or all of the memory identified by mlockall could not be locked")]
    CouldNotLockMemoryError,
    #[error("The flags argument is zero, or includes unimplemented flags")]
    InvalidFlagsError,
    #[error("Could not lock the needed amount of memory")]
    TooMuchMemoryToLockError,
    #[error("Not enough permissions to lock the memory pages")]
    NoPermission,
    #[error("Some unknown (or impossible) mlockall error happened")]
    UnknownMlockallError
}

pub type Result<T> = std::result::Result<T, Error>;
