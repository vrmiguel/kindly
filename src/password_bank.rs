use std::{borrow::Cow, ffi::CStr, marker::PhantomData, ptr::NonNull};

use libc::{c_char, getpwuid, getspnam};

use crate::error::{Error, Result};

/// Gets the effective user ID of the calling process
fn effective_user_id() -> u32 {
    // Safety: the POSIX Programmer's Manual states that
    // geteuid will always be successful.
    unsafe { libc::geteuid() }
}

pub struct PasswordEntry<'a> {
    username: NonNull<c_char>,
    password: NonNull<c_char>,
    spook: PhantomData<&'a c_char>,
}

pub struct PasswordBank;

impl PasswordBank {
    /// Queries the password bank (/dev/passwd) through the effective user id of the caller.
    ///
    /// Returns an entry with the username and password
    pub fn query_password_entry<'a>() -> Result<PasswordEntry<'static>> {
        let uid = effective_user_id();

        let passwd = unsafe { getpwuid(uid) };

        if !passwd.is_null() {
            // If getpwuid succeeded, let's get our data from it

            // Safety: we just checked that `passwd` is not NUlL
            let pw_name = unsafe { (*passwd).pw_name };
            let pw_passwd = unsafe { (*passwd).pw_passwd };

            // let username = unsafe { CStr::from_ptr(pw_name) };
            // let password = unsafe { CStr::from_ptr(pw_passwd) };

            return PasswordEntry::from_ptrs(pw_name, pw_passwd).ok_or(Error::PasswordBank);
        }

        Err(Error::PasswordBank)
    }

    pub fn query_shadow_file_by_username<'a>(username: NonNull<c_char>) -> Result<PasswordEntry<'static>> {
        let shadow_entry = unsafe { getspnam(username.as_ptr()) };

        dbg!(shadow_entry.is_null());

        if !shadow_entry.is_null() {
            // If getpwnam succeeded, let's get our data from it

            // Safety: we just checked that `passwd` is not NUlL
            let username = unsafe { (*shadow_entry).sp_namp };
            let password = unsafe { (*shadow_entry).sp_namp };

            return PasswordEntry::from_ptrs(username, password).ok_or(Error::ShadowFile);
        }

        Err(Error::ShadowFile)
    }
}

impl<'a> PasswordEntry<'a> {
    pub fn from_ptrs(username: *const c_char, password: *const c_char) -> Option<Self> {
        Some(Self {
            username: NonNull::new(username as *mut _)?,
            password: NonNull::new(password as *mut _)?,
            spook: PhantomData,
        })
    }

    pub fn username_ptr(&self) -> NonNull<c_char> {
        self.username
    }

    pub fn username(&self) -> &'_ CStr {
        unsafe { CStr::from_ptr(self.username.as_ptr()) }
    }

    pub fn username_utf8(&self) -> Cow<'_, str> {
        self.username().to_string_lossy()
    }

    pub fn password(&self) -> &'_ CStr {
        unsafe { CStr::from_ptr(self.password.as_ptr()) }
    }

    pub fn password_bytes(&self) -> &'_ [u8] {
        self.password().to_bytes()
    }

    pub fn password_is_one_char(&self) -> bool {
        self.password_bytes().len() == 1
    }
}
