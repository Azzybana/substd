use crate::streams::strings::StringStream;

pub struct Stdout {
    inner: StringStream,
}

impl Stdout {
    pub fn new() -> Self {
        Self {
            inner: StringStream::new(),
        }
    }
}
