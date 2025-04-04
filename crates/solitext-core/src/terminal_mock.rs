//! Mock implementation of terminal traits for testing

use std::io::{Write, Result};
use crate::terminal::{
    Key, Color, Terminal, TerminalInput, 
    Black, Blue, Cyan, Green, LightBlack, LightBlue, LightCyan, LightGreen, 
    LightMagenta, LightRed, LightWhite, LightYellow, Magenta, Red, Reset, White, Yellow
};

/// A mock stdout for testing
pub struct MockStdout {
    buffer: Vec<u8>,
}

impl MockStdout {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Write for MockStdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Default for MockStdout {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock raw terminal for testing
pub struct MockRawTerminal {
    inner: MockStdout,
}

impl Write for MockRawTerminal {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }
    
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

/// Terminal implementation for testing
impl Terminal for MockStdout {
    type RawTerminal = MockRawTerminal;
    
    fn into_raw_mode(self) -> Result<Self::RawTerminal> {
        Ok(MockRawTerminal { inner: self })
    }
    
    fn goto(x: u16, y: u16) -> String {
        format!("GOTO({},{})", x, y)
    }
    
    fn hide() -> String {
        "HIDE_CURSOR".to_string()
    }
    
    fn show() -> String {
        "SHOW_CURSOR".to_string()
    }
    
    fn clear_all() -> String {
        "CLEAR_ALL".to_string()
    }
}

/// Mock Color implementations for testing
macro_rules! impl_color {
    ($type:ty, $name:expr) => {
        impl Color for $type {
            fn fg_code(&self) -> String {
                format!("FG({})", $name)
            }
            
            fn bg_code(&self) -> String {
                format!("BG({})", $name)
            }
        }
    };
}

impl_color!(Black, "Black");
impl_color!(Red, "Red");
impl_color!(Green, "Green");
impl_color!(Yellow, "Yellow");
impl_color!(Blue, "Blue");
impl_color!(Magenta, "Magenta");
impl_color!(Cyan, "Cyan");
impl_color!(White, "White");
impl_color!(LightBlack, "LightBlack");
impl_color!(LightRed, "LightRed");
impl_color!(LightGreen, "LightGreen");
impl_color!(LightYellow, "LightYellow");
impl_color!(LightBlue, "LightBlue");
impl_color!(LightMagenta, "LightMagenta");
impl_color!(LightCyan, "LightCyan");
impl_color!(LightWhite, "LightWhite");
impl_color!(Reset, "Reset");

/// Mock input for testing
pub struct MockInput {
    keys: Vec<Key>,
    current_index: usize,
}

impl MockInput {
    pub fn new(keys: Vec<Key>) -> Self {
        Self {
            keys,
            current_index: 0,
        }
    }
    
    pub fn empty() -> Self {
        Self {
            keys: Vec::new(),
            current_index: 0,
        }
    }
    
    pub fn with_default_sequence() -> Self {
        Self::new(vec![
            Key::Char('1'),
            Key::Right,
            Key::Down,
            Key::Enter,
            Key::Char(' '),
            Key::Esc,
        ])
    }
}

impl TerminalInput for MockInput {
    type Keys = Self;
    
    fn keys(self) -> Self::Keys {
        self
    }
    
    fn read_key(keys: &mut Self::Keys) -> Option<Result<Key>> {
        if keys.current_index < keys.keys.len() {
            let key = keys.keys[keys.current_index];
            keys.current_index += 1;
            Some(Ok(key))
        } else {
            None
        }
    }
} 