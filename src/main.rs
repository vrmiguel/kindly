mod command;
mod const_time;
mod crypt;
mod drop_zeroed;
mod errno;
mod error;
mod input;
mod memory_lock;
mod password_bank;

use command::run_command;
use error::{Error, Result};
use libc::setuid;
use memory_lock::lock_memory_pages;
use password_bank::{effective_user_id, PasswordBank};

use crate::{const_time::VolatileBytes, drop_zeroed::DropZeroed, input::ask_for_password};

fn try_main() -> Result<i32> {
    let mut args = argv::iter().skip(1).peekable();
    let no_args_passed = args.peek().is_none();

    if no_args_passed {
        return Err(Error::NoCommandToRun);
    }

    let effective_user_id = effective_user_id();

    // Locks all pages mapped into the address space of the calling process.
    lock_memory_pages()?;

    // We'll query the password bank (/etc/passwd) for the entry corresponding to the user
    // that started `kindly`
    let (uid, mut pw_entry) = PasswordBank::query_password_entry()?;

    if pw_entry.password_is_one_char() {
        // When the password is one-char long (typically 'x'), that means that the actual
        // encrypted password is located in `/etc/shadow` instead of `/etc/passwd`.

        // We'll query the shadow file by username in order to get the actual encrypted password of the user
        // that started kindly
        pw_entry = PasswordBank::query_shadow_file_by_username(pw_entry.username_ptr())?;
    }

    // Asks for the user's password
    let password = ask_for_password(pw_entry.username_utf8()).ok_or(Error::PasswordAsking)?;

    // Encrypts the password in order to match the encrypted password entry in the password bank or shadow file
    let encrypted = crypt::encrypt(&password, pw_entry.password());

    // Zeroes the password in-memory and drops it
    password.drop_zeroed();

    let passwords_match = {
        // The user-supplied password that is now encrypted
        let encrypted = VolatileBytes::new(encrypted.as_bytes());

        // The encrypted password value found in the password bank or in the shadow file
        let password_from_entry = VolatileBytes::new(pw_entry.password_bytes());

        // We'll compare them through a "secure" `memeq` implementation
        encrypted == password_from_entry
    };

    // The supplied password is not correct!
    if !passwords_match {
        return Err(Error::Authentication);
    }

    // Elevate the privileges of the running process
    if unsafe { setuid(effective_user_id) } != 0 {
        return Err(Error::Setuid);
    }

    // Runs the command given through command-line arguments and waits
    // for it to exit
    let status = run_command(args)?;

    // Deescalate the privileges of the running process
    if unsafe { setuid(uid) } != 0 {
        return Err(Error::Setuid);
    }

    if !status.is_ok() {
        // The spawned process was signaled or terminated normally with a non-zero exit code
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
