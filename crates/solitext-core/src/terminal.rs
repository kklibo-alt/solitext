//! Terminal abstraction module
//! 
//! This module provides a platform-agnostic interface for terminal operations.
//! It defines traits that can be implemented by different terminal backends.
//! 
//! The main components are:
//! - The `Key` enum representing keyboard input
//! - Color-related types and the `Color` trait
//! - The `Terminal` trait for basic terminal operations
//! - The `TerminalInput` trait for handling user input
//!
//! # Adding a new backend
//! 
//! To add support for a new terminal backend:
//! 
//! 1. Create a new implementation of the `Terminal` trait
//! 2. Implement the `Color` trait for all color types
//! 3. Implement the `TerminalInput` trait for your input handling
//! 4. Wire up your implementation in your crate's main.rs
//!
//! See the implementations in solitext-local and solitext-web for examples.

// Platform-agnostic terminal key definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Alt(char),
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Backspace,
    Delete,
    Enter,
    Tab,
    Esc,
    F(u8),
    Null,
    Invalid,
}

/// Trait for color handling
pub trait Color {
    /// Get the foreground color code as a string
    fn fg_code(&self) -> String;
    
    /// Get the background color code as a string
    fn bg_code(&self) -> String;
}

// Standard color definitions
/// Black color
#[derive(Clone, Copy)]
pub struct Black;

/// Red color
#[derive(Clone, Copy)]
pub struct Red;

/// Green color
#[derive(Clone, Copy)]
pub struct Green;

/// Yellow color
#[derive(Clone, Copy)]
pub struct Yellow;

/// Blue color
#[derive(Clone, Copy)]
pub struct Blue;

/// Magenta color
#[derive(Clone, Copy)]
pub struct Magenta;

/// Cyan color
#[derive(Clone, Copy)]
pub struct Cyan;

/// White color
#[derive(Clone, Copy)]
pub struct White;

/// Light black (gray) color
#[derive(Clone, Copy)]
pub struct LightBlack;

/// Light red color
#[derive(Clone, Copy)]
pub struct LightRed;

/// Light green color
#[derive(Clone, Copy)]
pub struct LightGreen;

/// Light yellow color
#[derive(Clone, Copy)]
pub struct LightYellow;

/// Light blue color
#[derive(Clone, Copy)]
pub struct LightBlue;

/// Light magenta color
#[derive(Clone, Copy)]
pub struct LightMagenta;

/// Light cyan color
#[derive(Clone, Copy)]
pub struct LightCyan;

/// Light white color
#[derive(Clone, Copy)]
pub struct LightWhite;

/// Reset color to default
#[derive(Clone, Copy)]
pub struct Reset;

/// Terminal interface for cursor and screen operations
pub trait Terminal {
    /// The type used for raw terminal mode
    type RawTerminal;
    
    /// Put the terminal into raw mode
    fn into_raw_mode(self) -> std::io::Result<Self::RawTerminal>;
    
    /// Get the cursor goto escape sequence
    fn goto(x: u16, y: u16) -> String;
    
    /// Get the cursor hide escape sequence
    fn hide() -> String;
    
    /// Get the cursor show escape sequence
    fn show() -> String;
    
    /// Get the clear screen escape sequence
    fn clear_all() -> String;
}

/// Terminal input handling
pub trait TerminalInput {
    /// The type used for key iteration
    type Keys;
    
    /// Get an iterator over keys
    fn keys(self) -> Self::Keys;
    
    /// Read the next key from the input stream
    fn read_key(keys: &mut Self::Keys) -> Option<std::io::Result<Key>>;
}

// Add a re-export module for the terminal implementations
pub mod adapters {
    //! This module contains marker traits and type definitions needed for terminal adapters.
    //! These are used to overcome Rust's orphan rules when implementing traits for external types.
    
    use super::*;
    
    /// Marker trait for terminal color implementations
    pub trait ColorProvider {
        /// Get the foreground color code as a string
        fn fg_code(&self) -> String;
        
        /// Get the background color code as a string
        fn bg_code(&self) -> String;
    }
    
    /// Wrapper struct for color implementations
    #[derive(Clone, Copy)]
    pub struct ColorWrapper<T>(pub T);
    
    // Automatically implement Color for any type that implements ColorProvider
    impl<T: ColorProvider> Color for ColorWrapper<T> {
        fn fg_code(&self) -> String {
            self.0.fg_code()
        }
        
        fn bg_code(&self) -> String {
            self.0.bg_code()
        }
    }
    
    /// Marker trait for terminal implementations
    pub trait TerminalProvider {
        /// The type used for raw terminal mode
        type RawTerminal;
        
        /// Put the terminal into raw mode
        fn into_raw_mode(self) -> std::io::Result<Self::RawTerminal>;
        
        /// Get the cursor goto escape sequence
        fn goto(x: u16, y: u16) -> String;
        
        /// Get the cursor hide escape sequence
        fn hide() -> String;
        
        /// Get the cursor show escape sequence
        fn show() -> String;
        
        /// Get the clear screen escape sequence
        fn clear_all() -> String;
    }
    
    /// Wrapper struct for terminal implementations
    pub struct TerminalWrapper<T>(pub T);
    
    impl<T: TerminalProvider> Terminal for TerminalWrapper<T> {
        type RawTerminal = T::RawTerminal;
        
        fn into_raw_mode(self) -> std::io::Result<Self::RawTerminal> {
            self.0.into_raw_mode()
        }
        
        fn goto(x: u16, y: u16) -> String {
            T::goto(x, y)
        }
        
        fn hide() -> String {
            T::hide()
        }
        
        fn show() -> String {
            T::show()
        }
        
        fn clear_all() -> String {
            T::clear_all()
        }
    }
    
    impl<T> Default for TerminalWrapper<T> where T: Default {
        fn default() -> Self {
            TerminalWrapper(T::default())
        }
    }
    
    impl<T: std::io::Write> std::io::Write for TerminalWrapper<T> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.write(buf)
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            self.0.flush()
        }
    }
} 