use std::{borrow::Cow, ffi::CStr, marker::PhantomData, ptr::NonNull};

use libc::{c_char, getpwuid, getspnam};

use crate::error::{Error, Result};

/// Gets the user ID of the calling user
fn calling_user_id() -> u32 {
    // Safety: the POSIX Programmer's Manual states that
    // getuid will always be successful.
    unsafe { libc::getuid() }
}

pub fn effective_user_id() -> u32 {
    // Safety: the POSIX Programmer's Manual states that
    // geteuid will always be successful.
    unsafe { libc::geteuid() }
}

/// A reduced view of an entry in the password bank or in the shadow file
pub struct PasswordEntry<'a> {
    username: NonNull<c_char>,
    password: NonNull<c_char>,
    spook: PhantomData<&'a c_char>,
}

/// Abstracts over access to the password bank or to the shadow file
pub struct PasswordBank;

impl PasswordBank {
    /// Queries the password bank (/dev/passwd) through the effective user id of the caller.
    ///
    /// Returns an entry with the username and password
    pub fn query_password_entry() -> Result<(u32, PasswordEntry<'static>)> {
        let uid = calling_user_id();

        let passwd = unsafe { getpwuid(uid) };

        if !passwd.is_null() {
            // If getpwuid succeeded, let's get our data from it

            // Safety: we just checked that `passwd` is not NULL
            let uid = unsafe { (*passwd).pw_uid };
            let pw_name = unsafe { (*passwd).pw_name };
            let pw_passwd = unsafe { (*passwd).pw_passwd };

            let password_entry =
                PasswordEntry::from_ptrs(pw_name, pw_passwd).ok_or(Error::PasswordBank)?;

            return Ok((uid, password_entry));
        }

        Err(Error::PasswordBank)
    }

    pub fn query_shadow_file_by_username(
        username: NonNull<c_char>,
    ) -> Result<PasswordEntry<'static>> {
        let shadow_entry = unsafe { getspnam(username.as_ptr()) };

        if !shadow_entry.is_null() {
            // If getpwnam succeeded, let's get our data from it

            // Safety: we just checked that `passwd` is not NUlL
            let username = unsafe { (*shadow_entry).sp_namp };
            let password = unsafe { (*shadow_entry).sp_pwdp };

            return PasswordEntry::from_ptrs(username, password).ok_or(Error::ShadowFile);
        }

        Err(Error::ShadowFile)
    }
}

impl<'a> PasswordEntry<'a> {
    /// Instantiates a [`PasswordEntry`] from the raw pointers representing the username and the password
    pub fn from_ptrs(username: *const c_char, password: *const c_char) -> Option<Self> {
        Some(Self {
            username: NonNull::new(username as *mut _)?,
            password: NonNull::new(password as *mut _)?,
            spook: PhantomData,
        })
    }

    /// Returns the raw pointer of the username of this entry
    pub fn username_ptr(&self) -> NonNull<c_char> {
        self.username
    }

    /// Returns the username of this entry as a [`CStr`]
    pub fn username(&self) -> &'_ CStr {
        unsafe { CStr::from_ptr(self.username.as_ptr()) }
    }

    /// Returns the username of this entry in valid UTF-8
    pub fn username_utf8(&self) -> Cow<'_, str> {
        self.username().to_string_lossy()
    }

    /// Returns the password of this entry as a [`CStr`]
    pub fn password(&self) -> &'_ CStr {
        unsafe { CStr::from_ptr(self.password.as_ptr()) }
    }

    /// Returns a byte slice of the password of this entry
    pub fn password_bytes(&self) -> &'_ [u8] {
        self.password().to_bytes()
    }

    /// Returns true if the password is one char in length
    pub fn password_is_one_char(&self) -> bool {
        self.password_bytes().len() == 1
    }
}
