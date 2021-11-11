use std::{borrow::Cow, ffi::CStr, mem::MaybeUninit, ptr};

use libc::{c_char, getpwuid_r, passwd};

use crate::error::{Error, Result};

/// Gets the effective user ID of the calling process
fn effective_user_id() -> u32 {
    // Safety: the POSIX Programmer's Manual states that
    // geteuid will always be successful.
    unsafe { libc::geteuid() }
}

pub struct PasswordEntry<'a> {
    username: Cow<'a, str>,
    password: &'a CStr
}

impl<'a> PasswordEntry<'a> {
    pub fn username(&self) -> &Cow<'_, str> {
        &self.username
    }

    pub fn password_bytes(&self) -> &'_ [u8] {
        self.password.to_bytes()
    }
}

pub fn query_password_entry<'a>(buf: &'a mut [c_char]) -> Result<PasswordEntry<'a>> {

    let uid = effective_user_id();

    let mut result = ptr::null_mut();

    // Safety: the all-zero byte pattern is a valid struct passwd
    let mut passwd = MaybeUninit::<passwd>::zeroed();

    let status = unsafe {
        getpwuid_r(
            uid,
            passwd.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.len(),
            &mut result,
        )
    };

    if status == 0 && !result.is_null() {
        // If getpwuid_r succeeded, let's get the username from it

        

        let pw_name = unsafe { (*passwd.as_ptr()).pw_name };
        let pw_passwd = unsafe { (*passwd.as_ptr()).pw_passwd };

        // TODO: I'm not actually sure how long this pointer is valid for
        let username =
            unsafe { CStr::from_ptr(pw_name) }.to_string_lossy();
        
        let password = 
            unsafe { CStr::from_ptr(pw_passwd) };

        return Ok(PasswordEntry {
            username,
            password
        });
    }

    Err(Error::UnknownUsername)
}