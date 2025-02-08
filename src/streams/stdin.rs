use crate::streams::_buffer::Buffer;
use crate::streams::strings::StringStream; // new import

/// A minimal Stdin supporting blocking/non-blocking reads and signal handling.
pub struct Stdin {
    inner: StringStream,
}

impl Stdin {
    /// Creates a new `Stdin`.
    pub fn new() -> Self {
        Self {
            inner: StringStream::new(),
        }
    }

    /// Performs a blocking read.
    ///
    /// In this minimal version, it returns a clone of the input buffer.
    pub fn read_blocking(&mut self) -> Option<String> {
        Some(self.inner.buffer.clone())
    }

    /// Performs a non-blocking read.
    ///
    /// Immediately returns the current buffer.
    pub fn read_nonblocking(&mut self) -> Option<String> {
        Some(self.inner.buffer.clone())
    }

    /// Handles an input signal, e.g., interruption (Ctrl+C).
    ///
    /// Dummy implementation for a no-std environment.
    pub fn handle_signal(&self) {
        // No-op in minimal implementation.
    }
}

// Implement Buffer for Stdin.
impl Buffer for Stdin {
    type Item = String;
    fn into_buffer(self) -> Vec<Self::Item> {
        vec![self.inner.buffer]
    }
}
