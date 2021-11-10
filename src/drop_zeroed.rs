use std::{ptr, sync::atomic};

use unixstring::UnixString;

pub trait DropZeroed {
    fn drop_zeroed(self);
}

impl DropZeroed for UnixString {
    fn drop_zeroed(self) {
        let mut bytes = self.into_bytes();
        for item in &mut bytes {
            unsafe { ptr::write_volatile(item as *mut _, 0_u8) }
            atomic::compiler_fence(atomic::Ordering::SeqCst);
        }
    }
}
