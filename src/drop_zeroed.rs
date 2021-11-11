use std::{ptr, sync::atomic};

use unixstring::UnixString;

pub trait DropZeroed {
    fn drop_zeroed(self);
}

impl DropZeroed for Vec<u8> {
    fn drop_zeroed(self) {
        let mut bytes = self;
        for byte in &mut bytes {
            unsafe { ptr::write_volatile(byte as *mut _, 0_u8) }
            atomic::compiler_fence(atomic::Ordering::SeqCst);
        }
    }
}


impl DropZeroed for String {
    fn drop_zeroed(self) {
        self.into_bytes().drop_zeroed();
    }
}

impl DropZeroed for UnixString {
    fn drop_zeroed(self) {
        self.into_bytes().drop_zeroed()
    }
}