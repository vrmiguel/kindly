use unixstring::UnixString;

use crate::error::{Error, Result};

/// Safely reads the password from a terminal
pub fn ask_for_password(username: impl AsRef<str>) -> Result<UnixString> {
    print!("[kindly] Password for {}:", username.as_ref());

    let password = rpassword::read_password_from_tty(None)?;

    let password = UnixString::try_from(password).map_err(|_| Error::UnixString)?;

    Ok(password)
}
