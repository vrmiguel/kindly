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

use crate::{const_time::VolatileBytes, drop_zeroed::DropZeroed, input::ask_for_password};

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

    let password = ask_for_password(pw_entry.username_utf8())?;

    let encrypted = crypt::encrypt(&password, pw_entry.password());

    // Zeroes the password in-memory and drops it
    password.drop_zeroed();

    println!("encrypted: {}", encrypted.as_path().display());
    println!("passwd->pwd: {}", pw_entry.password().to_string_lossy());

    let passwords_match = {
        // The user-supplied password that is now encrypted
        let encrypted = VolatileBytes::new(encrypted.as_bytes_with_nul());

        // The encrypted password value found in the password bank or in the shadow file
        let password_from_entry = VolatileBytes::new(pw_entry.password_bytes());

        // We'll compare them through a "secure" `memeq` implementation
        encrypted == password_from_entry
    };

    if !passwords_match {
        return Err(Error::Authentication);
    }

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
