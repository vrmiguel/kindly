use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::{ffi::OsStr, process::Command};

use crate::error::{Error, Result};

pub enum TerminationStatus {
    Signaled(i32),
    TerminatedNormally(i32),
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

pub fn run_command<'a>(mut args: impl Iterator<Item = &'a OsStr>) -> Result<TerminationStatus> {
    let command_name = args.next().ok_or(Error::NoCommandToRun)?;

    let mut child = Command::new(command_name).args(args).spawn()?;

    let exit_status = child.wait()?;

    Ok(exit_status.into())
}
