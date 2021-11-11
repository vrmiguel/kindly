//! What `kindly` needs to run commands

use std::fmt::Display;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::{ffi::OsStr, process::Command};

use crate::error::{Error, Result};

/// Represents the termination status of a command
pub enum TerminationStatus {
    /// If the given command was interrupted through a signal, this variant holds the value of that signal
    Signaled(i32),
    /// If the command terminated normally (i.e. not signal interrupted), this variant holds the exit code of the command
    TerminatedNormally(i32),
}

impl TerminationStatus {
    /// Returns true if the command terminated normally and with exit code 0
    pub fn is_ok(&self) -> bool {
        matches!(self, TerminationStatus::TerminatedNormally(0))
    }

    /// Returns either the code of this command (if terminated normally) or the value of the signal that killed it (if signal interrupted)
    pub fn code_or_signal(&self) -> i32 {
        match self {
            TerminationStatus::Signaled(signal) => *signal,
            TerminationStatus::TerminatedNormally(code) => *code,
        }
    }
}

impl Display for TerminationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminationStatus::Signaled(signal) => {
                write!(f, "The command was terminated from signal {}", signal)
            }
            TerminationStatus::TerminatedNormally(exit_code) => write!(
                f,
                "The command terminated normally with exit code {}",
                exit_code
            ),
        }
    }
}

impl From<ExitStatus> for TerminationStatus {
    fn from(exit_status: ExitStatus) -> Self {
        match exit_status.signal() {
            Some(signal) => TerminationStatus::Signaled(signal),
            None => {
                // Safety: the docs state that, on Unix, ExitStatus::code will only fail if
                //         the process was killed from a signal. We've just checked that this is not the case.
                let exit_code = exit_status.code().unwrap();
                TerminationStatus::TerminatedNormally(exit_code)
            }
        }
    }
}

/// Simple abstraction over [`Command`] that spawns the command given by `args`, waits for its exit and returns a [`TerminationStatus`].
pub fn run_command<'a>(mut args: impl Iterator<Item = &'a OsStr>) -> Result<TerminationStatus> {
    let command_name = args.next().ok_or(Error::NoCommandToRun)?;

    let mut child = Command::new(command_name).args(args).spawn()?;

    let exit_status = child.wait()?;

    Ok(exit_status.into())
}
