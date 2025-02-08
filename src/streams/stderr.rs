use crate::streams::_buffer::Buffer;
use crate::streams::strings::StringStream; // new import

/// A minimal Stderr that buffers error output and flushes immediately.
pub struct Stderr {
    inner: StringStream,
}

impl Stderr {
    /// Creates a new `Stderr`.
    pub fn new() -> Self {
        Self {
            inner: StringStream::new(),
        }
    }

    /// Flushes the error buffer immediately.
    ///
    /// In this minimal version, flushing simply clears the buffer.
    pub fn flush(&mut self) -> Result<(), ()> {
        self.inner.buffer.clear();
        Ok(())
    }
}

// Implement Buffer for Stderr.
impl Buffer for Stderr {
    type Item = String;
    fn into_buffer(self) -> Vec<Self::Item> {
        vec![self.inner.buffer]
    }
}
