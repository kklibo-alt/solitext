use std::io::{self, Write};

/// A trait representing terminal functionality.
/// This trait abstracts the functionality provided by the termion crate
/// to allow for alternative implementations.
pub trait Terminal: Write {
    /// Set foreground color
    fn set_fg(&mut self, color: TerminalColor) -> io::Result<()>;
    
    /// Set background color
    fn set_bg(&mut self, color: TerminalColor) -> io::Result<()>;
    
    /// Reset foreground color to default
    fn reset_fg(&mut self) -> io::Result<()>;
    
    /// Reset background color to default
    fn reset_bg(&mut self) -> io::Result<()>;
    
    /// Clear the entire screen
    fn clear_screen(&mut self) -> io::Result<()>;
    
    /// Move cursor to a specific position (1-indexed)
    fn goto(&mut self, x: u16, y: u16) -> io::Result<()>;
    
    /// Hide the cursor
    fn hide_cursor(&mut self) -> io::Result<()>;
    
    /// Show the cursor
    fn show_cursor(&mut self) -> io::Result<()>;
    
    /// Enter raw mode where keypresses are immediately available
    fn enter_raw_mode(&mut self) -> io::Result<()>;
    
    /// Exit raw mode
    fn exit_raw_mode(&mut self) -> io::Result<()>;
    
    /// Flush any buffered output
    fn flush(&mut self) -> io::Result<()>;
}

/// Colors available in the terminal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalColor {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}

/// Key events from the terminal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Char(char),
    Ctrl(char),
    Alt(char),
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Backspace,
    Delete,
    Insert,
    Esc,
    F(u8),
    Null,
}

/// Iterator for reading keys from stdin
pub trait TerminalKeys {
    /// Read the next key event from the terminal
    fn next_key(&mut self) -> io::Result<Option<KeyEvent>>;
    
    /// Create an iterator that yields key events
    fn keys(&mut self) -> KeysIter<'_, Self>
    where
        Self: Sized,
    {
        KeysIter { source: self }
    }
}

/// Iterator for key events
pub struct KeysIter<'a, T: TerminalKeys + ?Sized> {
    source: &'a mut T,
}

impl<'a, T: TerminalKeys + ?Sized> Iterator for KeysIter<'a, T> {
    type Item = io::Result<KeyEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next_key() {
            Ok(Some(k)) => Some(Ok(k)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// Factory for creating terminal implementations
pub trait TerminalFactory {
    /// Type of terminal produced by this factory
    type Terminal: Terminal;
    
    /// Type of key source produced by this factory
    type Keys: TerminalKeys;
    
    /// Create a new terminal
    fn create_terminal(&self) -> io::Result<Self::Terminal>;
    
    /// Create a new key source
    fn create_keys(&self) -> Self::Keys;
} 