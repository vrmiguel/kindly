use unixstring::UnixString;

use crate::drop_zeroed::DropZeroed;

/// Safely reads the password from a terminal
pub fn ask_for_password(username: impl AsRef<str>) -> Option<UnixString> {
    println!("[kindly] Password for {}:", username.as_ref());

    let password = rpassword::read_password_from_tty(None).ok()?;

    // As pointed out by @ferrouille, we must not use `UnixString::try_from` to convert here
    // since it could lead to a copy of the password being left unzeroed somewhere in the memory.
    let mut unx = UnixString::with_capacity(password.len());

    let push_worked = unx.push_bytes(password.as_bytes()).is_ok();

    password.drop_zeroed();
    
    if push_worked {
        Some(unx)
    } else {
        unx.drop_zeroed();
        None
    }
}
