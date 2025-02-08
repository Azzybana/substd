use crate::streams::strings::StringStream;

pub struct Stderr {
    inner: StringStream,
}

impl Stderr {
    pub fn new() -> Self {
        Self {
            inner: StringStream::new(),
        }
    }
}
