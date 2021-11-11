use std::{borrow::Cow, ffi::CStr, mem::MaybeUninit, ptr};

use libc::{c_char, getpwuid_r, passwd};

use crate::error::{Error, Result};

/// Gets the effective user ID of the calling process
fn effective_user_id() -> u32 {
    // Safety: the POSIX Programmer's Manual states that
    // geteuid will always be successful.
    unsafe { libc::geteuid() }
}

pub fn username<'a>(buf: &'a mut [c_char]) -> Result<Cow<'a, str>> {

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

        // TODO: I'm not actually sure how long this pointer is valid for
        let username =
            unsafe { CStr::from_ptr(pw_name) }.to_string_lossy();

        return Ok(username);
    }

    Err(Error::UnknownUsername)
}