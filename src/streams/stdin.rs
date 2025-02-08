use crate::streams::strings::StringStream;

pub struct Stdin {
    inner: StringStream,
}

impl Stdin {
    pub fn new() -> Self {
        Self {
            inner: StringStream::new(),
        }
    }
}
