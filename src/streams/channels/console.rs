use std::fmt;

pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Console {}
    }

    pub fn read_line(&self) -> String {
        // Dummy implementation. Replace with actual console input handling if needed.
        "dummy input".to_string()
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // For simulation, print to stdout.
        print!("{}", s);
        Ok(())
    }
}
