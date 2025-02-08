use crate::streams::strings::StringStream;

/// A minimal Stdout that buffers output and provides flushing.
pub struct Stdout {
    inner: StringStream,
}

impl Stdout {
    /// Creates a new `Stdout`.
    pub fn new() -> Self {
        Self {
            inner: StringStream::new(),
        }
    }

    /// Flushes the output buffer.
    ///
    /// In this minimal no-std implementation, flushing simply clears the buffer.
    pub fn flush(&mut self) -> Result<(), ()> {
        self.inner.buffer.clear();
        Ok(())
    }
}
