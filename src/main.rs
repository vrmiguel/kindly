mod command;
mod const_time;
mod crypt;
mod drop_zeroed;
mod error;
mod input;
mod password_bank;

use command::run_command;
use error::{Error, Result};
use libc::setreuid;
use password_bank::PasswordBank;

use crate::{drop_zeroed::DropZeroed, input::ask_for_password};

fn try_main() -> Result<i32> {
    let mut args = argv::iter().peekable();
    let no_args_passed = args.peek().is_none();

    if no_args_passed {
        return Err(Error::NoCommandToRun);
    }

    let mut pw_entry = PasswordBank::query_password_entry()?;

    if pw_entry.password_is_one_char() {
        // When the password is one-char long (typically 'x'), that means that the actual
        // encrypted password is located in `/etc/shadow` instead of `/etc/passwd`.
        // username.write(pw_entry.username());
        pw_entry = PasswordBank::query_shadow_file_by_username(pw_entry.username_ptr())?;
    }

    dbg!(pw_entry.password_bytes());

    let password = ask_for_password(pw_entry.username_utf8())?;

    let encrypted = crypt::encrypt(&password, pw_entry.password());

    // Zeroes the password in-memory and drops it
    password.drop_zeroed();

    println!("encrypted: {}", encrypted.as_path().display());

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
