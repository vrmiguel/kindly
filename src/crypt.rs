use std::ffi::CStr;

use libc::{c_char, free};
use unixstring::UnixString;

extern "C" {
    fn crypt(key: *const c_char, salt: *const c_char) -> *const c_char;
}

pub fn encrypt(key: impl AsRef<CStr>, salt: impl AsRef<CStr>) -> UnixString {
    let (key, salt) = (key.as_ref(), salt.as_ref());

    let encrypted_ptr = unsafe { crypt(key.as_ptr(), salt.as_ptr()) };

    let encrypted = unsafe { UnixString::from_ptr(encrypted_ptr) };

    encrypted
}
