use memsec::memeq;

/// A wrapper over a byte slice whose comparisons _should_
/// be immune to timing attacks
pub struct VolatileBytes<'a> {
    bytes: &'a [u8],
}

impl<'a> VolatileBytes<'a> {
    /// Wraps over a byte slice
    ///
    /// Panics if the byte slice is empty
    pub fn new(bytes: &'a [u8]) -> Self {
        assert!(!bytes.is_empty());
        Self { bytes }
    }

    /// Returns the amount of bytes in the inner slice
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl AsRef<[u8]> for VolatileBytes<'_> {
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl PartialEq for VolatileBytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: slice length checking is O(1) so this likely cannot
        // create a timing attack, but I need to investigate further in order to assure that
        if self.len() != other.len() {
            return false;
        }

        let length = self.len();

        dbg!(unsafe { memeq(self.as_ref().as_ptr(), other.as_ref().as_ptr(), length) })
    }
}
