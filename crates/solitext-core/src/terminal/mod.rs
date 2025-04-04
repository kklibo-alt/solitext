//! Terminal abstraction module

pub mod termion_impl;

use std::any::Any;
use std::io::Write;

/// Terminal interface trait that abstracts all terminal operations
pub trait Terminal: Write {
    /// Create a new terminal instance
    fn new() -> Self where Self: Sized;
    
    /// Clear the screen
    fn clear_screen(&mut self);
    
    /// Hide the cursor
    fn hide_cursor(&mut self);
    
    /// Show the cursor
    fn show_cursor(&mut self);
    
    /// Move cursor to specific position (column, row)
    fn goto(&mut self, col: u16, row: u16);
    
    /// Set foreground and background colors
    fn set_colors(&mut self, fg: &dyn TerminalColor, bg: &dyn TerminalColor);
    
    /// Reset colors to default
    fn reset_colors(&mut self);
    
    /// Get the default background color
    fn default_bg(&self) -> Box<dyn TerminalColor>;
    
    /// Get the default foreground color
    fn default_fg(&self) -> Box<dyn TerminalColor>;
    
    /// Read a key from input
    fn read_key(&self) -> Option<Key>;
    
    /// Get an iterator over key presses
    fn keys(&self) -> Box<dyn Iterator<Item = Result<Key, std::io::Error>> + '_>;
    
    /// Wait for and read a single key press
    fn read_single_key(&self) -> Option<Key> {
        self.keys().next().and_then(|r| r.ok())
    }
}

/// Color abstraction trait
pub trait TerminalColor: Send + Sync {
    /// Return a color that is Red
    fn red() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Black
    fn black() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is White
    fn white() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Light White
    fn light_white() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Light Green
    fn light_green() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Light Black
    fn light_black() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Light Red
    fn light_red() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Light Blue
    fn light_blue() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Yellow
    fn yellow() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is Light Yellow
    fn light_yellow() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Return a color that is reset color
    fn reset() -> Box<dyn TerminalColor> where Self: Sized;
    
    /// Check if this is a red color
    fn is_red(&self) -> bool;
    
    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Key event representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// Char represents a character key
    Char(char),
    /// Left arrow key
    Left,
    /// Right arrow key
    Right,
    /// Up arrow key
    Up,
    /// Down arrow key
    Down,
    /// Home key
    Home,
    /// End key
    End,
    /// Escape key
    Esc,
    /// Ctrl+c
    CtrlC,
} 