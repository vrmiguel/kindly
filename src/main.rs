mod auth;
mod command;
mod drop_zeroed;
mod input;
mod error;
mod user;

use command::run_command;
use error::{Error, Result};
use libc::c_char;
use user::username;

use crate::{drop_zeroed::DropZeroed, input::ask_for_password};

fn try_main() -> Result<i32> {

    let mut args = argv::iter().peekable();
    let no_args_passed = args.peek().is_none();

    if no_args_passed {
        return Err(Error::NoCommandToRun);
    }

    let mut buf: [c_char; 2048] = [0; 2048];

    let username = username(&mut buf)?;

    let password = ask_for_password(username)?;
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
