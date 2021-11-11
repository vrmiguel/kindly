use crate::error::Result;

pub fn ask_for_password(username: impl AsRef<str>) -> Result<String> {
    println!("[kindly] Password for {}", username.as_ref());

    Ok(rpassword::read_password_from_tty(None)?)
}