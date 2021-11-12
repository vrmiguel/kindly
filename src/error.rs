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
    #[error("Failed to ask for password")]
    PasswordAsking,
    #[error("Authentication failed")]
    Authentication,
    #[error("Failed to set UID")]
    Setuid,
    #[error("Some or all of the memory identified by mlockall could not be locked")]
    CouldNotLockMemory,
    #[error("The flags argument is zero, or includes unimplemented flags")]
    InvalidFlags,
    #[error("Could not lock the needed amount of memory")]
    TooMuchMemoryToLock,
    #[error("Not enough permissions to lock the memory pages")]
    NoPermission,
    #[error("Some unknown (or impossible) mlockall error happened")]
    UnknownMlockall,
}

pub type Result<T> = std::result::Result<T, Error>;
