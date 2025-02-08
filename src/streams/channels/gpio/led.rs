use crate::streams::strings::StringStream;
use core::sync::atomic::spin_loop_hint;

/// Adjust this factor to calibrate the delay length per unit.
const UNIT_DELAY_FACTOR: u32 = 1000;

/// This static table maps characters (A-Z, 0-9) to their compact Morse code representations.
static MORSE_MAP: [(char, &str); 36] = [
    ('A', ".-"),
    ('B', "-..."),
    ('C', "-.-."),
    ('D', "-.."),
    ('E', "."),
    ('F', "..-."),
    ('G', "--."),
    ('H', "...."),
    ('I', ".."),
    ('J', ".---"),
    ('K', "-.-"),
    ('L', ".-.."),
    ('M', "--"),
    ('N', "-."),
    ('O', "---"),
    ('P', ".--."),
    ('Q', "--.-"),
    ('R', ".-."),
    ('S', "..."),
    ('T', "-"),
    ('U', "..-"),
    ('V', "...-"),
    ('W', ".--"),
    ('X', "-..-"),
    ('Y', "-.--"),
    ('Z', "--.."),
    ('0', "-----"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
];

/// Converts a character to its Morse code representation, if available.
/// Non-mapped characters are ignored.
fn char_to_morse(ch: char) -> Option<&'static str> {
    // Convert letters to uppercase.
    let uc = if ch.is_ascii_alphabetic() {
        ch.to_ascii_uppercase()
    } else {
        ch
    };
    for &(c, code) in MORSE_MAP.iter() {
        if c == uc {
            return Some(code);
        }
    }
    None
}

/// A minimal LED controller that uses a mutable boolean reference for output.
/// The LED state is represented as `true` for on and `false` for off.
pub struct Led<'a> {
    state: &'a mut bool,
}

impl<'a> Led<'a> {
    /// Creates a new `Led` with a mutable reference to an external boolean.
    /// This boolean represents the hardware LED state.
    pub fn new(state: &'a mut bool) -> Self {
        Self { state }
    }

    /// Turns the LED on.
    fn on(&mut self) {
        *self.state = true;
    }

    /// Turns the LED off.
    fn off(&mut self) {
        *self.state = false;
    }

    /// A simple busy-loop delay function.
    ///
    /// The delay is measured in "units", where one unit is the duration of a dot.
    fn delay(&self, units: u32) {
        for _ in 0..units * UNIT_DELAY_FACTOR {
            spin_loop_hint();
        }
    }

    /// Displays the given string (from a `StringStream`) as Morse code.
    ///
    /// Timing rules used:
    /// - Dot: LED on for 1 unit.
    /// - Dash: LED on for 3 units.
    /// - Intra-character gap: 1 unit off.
    /// - Inter-letter gap: 3 units off.
    /// - Inter-word gap: 7 units off.
    ///
    /// Invalid Morse characters are ignored.
    pub fn display(&mut self, stream: &StringStream) {
        let mut first_letter = true;
        for ch in stream.buffer.chars() {
            if ch == ' ' {
                // Word gap: 7 units off.
                self.delay(7);
                first_letter = true;
                continue;
            }
            if let Some(morse) = char_to_morse(ch) {
                if !first_letter {
                    // Letter gap: 3 units off.
                    self.delay(3);
                }
                first_letter = false;
                let mut first_symbol = true;
                for symbol in morse.chars() {
                    if !first_symbol {
                        // Intra-character gap: 1 unit off.
                        self.delay(1);
                    }
                    first_symbol = false;
                    match symbol {
                        '.' => {
                            self.on();
                            self.delay(1);
                            self.off();
                        }
                        '-' => {
                            self.on();
                            self.delay(3);
                            self.off();
                        }
                        _ => {} // Ignore any invalid symbol.
                    }
                }
            }
        }
    }
}
