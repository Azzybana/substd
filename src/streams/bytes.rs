pub struct ByteStream {
    pub buffer: Vec<u8>,
}

impl ByteStream {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }
}
