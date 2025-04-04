// Terminal module with platform-agnostic interfaces for terminal functionality

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

// Color trait
pub trait Color {
    fn fg_code(&self) -> String;
    fn bg_code(&self) -> String;
}

// Standard color definitions
pub struct Black;
pub struct Red;
pub struct Green;
pub struct Yellow;
pub struct Blue;
pub struct Magenta;
pub struct Cyan;
pub struct White;
pub struct LightBlack;
pub struct LightRed;
pub struct LightGreen;
pub struct LightYellow;
pub struct LightBlue;
pub struct LightMagenta;
pub struct LightCyan;
pub struct LightWhite;
pub struct Reset;

// Terminal interface for cursor and screen operations
pub trait Terminal {
    type RawTerminal;
    
    // Raw mode handling
    fn into_raw_mode(self) -> std::io::Result<Self::RawTerminal>;
    
    // Cursor operations
    fn goto(x: u16, y: u16) -> String;
    fn hide() -> String;
    fn show() -> String;
    
    // Screen operations
    fn clear_all() -> String;
}

// Terminal input handling
pub trait TerminalInput {
    type Keys;
    
    fn keys(self) -> Self::Keys;
    fn read_key(keys: &mut Self::Keys) -> Option<std::io::Result<Key>>;
} 