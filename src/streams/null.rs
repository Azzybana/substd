/// A null stream that ignores all I/O operations.
pub struct NullStream;

impl NullStream {
    /// Creates a new `NullStream`.
    ///
    /// Always returns a null stream that performs no operations.
    pub fn new() -> Self {
        // Returning a new instance of NullStream.
        NullStream
    }
}

impl Read for NullStream {
    /// Ignores the input argument and returns no data.
    fn read_str<T>(&self, _arg: T) -> Option<T> {
        None
    }
}

impl Write for NullStream {
    /// Ignores the input and simulates a successful write.
    fn write_str(&mut self, _arg: &T) {}
}
