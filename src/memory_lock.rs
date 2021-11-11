pub fn get_username(uid: u32) -> Option<String> {
    const BUF_SIZ: usize = 2048;
    let mut buf = MaybeUninit::<[c_char; BUF_SIZ]>::uninit();
    let mut result = ptr::null_mut();

    // Safety: the all-zero byte pattern is a valid struct passwd
    let mut passwd = MaybeUninit::<passwd>::zeroed();

    let status = unsafe {
        let buf_slice_ptr: *mut [c_char] = buf.as_mut_ptr();

        getpwuid_r(
            uid,
            passwd.as_mut_ptr(),
            buf_slice_ptr.as_mut_ptr(),
            buf_slice_ptr.len(),
            &mut result,
        )
    };

    if status == 0 && !result.is_null() {
        // If getpwuid_r succeeded, let's get the username from it

        let pw_name = (*passwd.as_ptr()).pw_name;

        let username =
            unsafe { UnixString::from_ptr(pw_name) }.into_string_lossy();

        return Some(username);
    }

    None
}