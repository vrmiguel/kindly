mod command;
mod drop_zeroed;
mod input;
mod error;
mod password_bank;

use command::run_command;
use error::{Error, Result};
use libc::c_char;
use password_bank::{query_password_entry};

use crate::{drop_zeroed::DropZeroed, input::ask_for_password};

fn try_main() -> Result<i32> {

    let mut buf: [c_char; 2048] = [0; 2048];

    let mut args = argv::iter().peekable();
    let no_args_passed = args.peek().is_none();

    if no_args_passed {
        return Err(Error::NoCommandToRun);
    }

    let pw_entry = query_password_entry(&mut buf)?;

    dbg!(pw_entry.password_bytes());

    let password = ask_for_password(pw_entry.username())?;
    password.drop_zeroed();

    let status = run_command(argv::iter().skip(1))?;

    if !status.is_ok() {
        println!("[kindly] {}", status);
    }

    Ok(status.code_or_signal())
}

fn main() {
    match try_main() {
        Ok(code_or_signal) => std::process::exit(code_or_signal),
        Err(err) => {
            println!("[kindly] {}", err);
            std::process::exit(127);
        }
    }
}
