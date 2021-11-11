use memsec::memeq;

pub struct VolatileBytes<'a> {
    bytes: &'a [u8],
}

impl<'a> VolatileBytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        assert!(!bytes.is_empty());
        Self { bytes }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl AsRef<[u8]> for VolatileBytes<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl PartialEq for VolatileBytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: slice length checking is O(1) so this likely cannot
        // create a timing attack, but I need to investigate further to check that
        if self.len() != other.len() {
            return false;
        }

        let length = self.len();

        unsafe { memeq(self.as_ref().as_ptr(), other.as_ref().as_ptr(), length) }
    }
}