pub struct StringStream {
    pub buffer: String,
}

impl StringStream {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

impl Write for StringStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

// Example usage at end of file:
#[cfg(test)]
mod tests {
    use super::*;
    use crate::streams::emitters::console::ConsoleEmitter;
    use crate::streams::sinks::console::ConsoleSink;

    #[test]
    fn example_io() {
        // Set up primitive I/O
        let mut input_stream = StringStream::new();
        let mut output_sink = ConsoleSink::new();

        // Emitter reads from a simulated console (dummy implementation)
        let mut emitter = ConsoleEmitter::new();
        input_stream.write_str(&emitter.read_line()).unwrap();

        // Write the input buffer to the console sink (stdout simulation)
        write!(output_sink, "Echo: {}", input_stream.buffer).unwrap();
    }
}
