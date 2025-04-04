use std::io::{Stdout, Write, stdin, stdout};
use solitext_core::terminal::{
    Black, Blue, Color, Cyan, Green, Key, LightBlack, LightBlue, LightCyan, LightGreen, 
    LightMagenta, LightRed, LightWhite, LightYellow, Magenta, Red, Reset, Terminal, 
    TerminalInput, White, Yellow
};
use termion;

// Map termion Keys to our Key enum
pub fn map_termion_key(key: termion::event::Key) -> Key {
    match key {
        termion::event::Key::Char(c) => Key::Char(c),
        termion::event::Key::Alt(c) => Key::Alt(c),
        termion::event::Key::Ctrl(c) => Key::Ctrl(c),
        termion::event::Key::Left => Key::Left,
        termion::event::Key::Right => Key::Right,
        termion::event::Key::Up => Key::Up,
        termion::event::Key::Down => Key::Down,
        termion::event::Key::Home => Key::Home,
        termion::event::Key::End => Key::End,
        termion::event::Key::Backspace => Key::Backspace,
        termion::event::Key::Delete => Key::Delete,
        termion::event::Key::Insert => Key::Invalid, // Not mapped in our Key enum
        termion::event::Key::PageUp => Key::Invalid, // Not mapped in our Key enum
        termion::event::Key::PageDown => Key::Invalid, // Not mapped in our Key enum
        termion::event::Key::F(n) => Key::F(n),
        termion::event::Key::Esc => Key::Esc,
        termion::event::Key::Null => Key::Null,
        _ => Key::Invalid,
    }
}

// Implement Color for each color type
impl Color for Black {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Black))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Black))
    }
}

impl Color for Red {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Red))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Red))
    }
}

impl Color for Green {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Green))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Green))
    }
}

impl Color for Yellow {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Yellow))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Yellow))
    }
}

impl Color for Blue {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Blue))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Blue))
    }
}

impl Color for Magenta {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Magenta))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Magenta))
    }
}

impl Color for Cyan {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Cyan))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Cyan))
    }
}

impl Color for White {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::White))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::White))
    }
}

impl Color for LightBlack {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightBlack))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightBlack))
    }
}

impl Color for LightRed {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightRed))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightRed))
    }
}

impl Color for LightGreen {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightGreen))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightGreen))
    }
}

impl Color for LightYellow {
    fn fg_code(&self) -> String {
        // Use Yellow since there's no LightYellow in termion
        format!("{}", termion::color::Fg(termion::color::Yellow))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Yellow))
    }
}

impl Color for LightBlue {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightBlue))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightBlue))
    }
}

impl Color for LightMagenta {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightMagenta))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightMagenta))
    }
}

impl Color for LightCyan {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightCyan))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightCyan))
    }
}

impl Color for LightWhite {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightWhite))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightWhite))
    }
}

impl Color for Reset {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Reset))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Reset))
    }
}

// Implement Terminal for Stdout
impl Terminal for Stdout {
    type RawTerminal = termion::raw::RawTerminal<Stdout>;
    
    fn into_raw_mode(self) -> std::io::Result<Self::RawTerminal> {
        termion::raw::IntoRawMode::into_raw_mode(self)
    }
    
    fn goto(x: u16, y: u16) -> String {
        format!("{}", termion::cursor::Goto(x, y))
    }
    
    fn hide() -> String {
        format!("{}", termion::cursor::Hide)
    }
    
    fn show() -> String {
        format!("{}", termion::cursor::Show)
    }
    
    fn clear_all() -> String {
        format!("{}", termion::clear::All)
    }
}

// Implement TerminalInput for Stdin
pub struct Stdin;

impl Stdin {
    pub fn new() -> Self {
        Stdin
    }
}

impl TerminalInput for Stdin {
    type Keys = termion::input::Keys<std::io::Stdin>;
    
    fn keys(self) -> Self::Keys {
        stdin().keys()
    }
    
    fn read_key(keys: &mut Self::Keys) -> Option<std::io::Result<Key>> {
        keys.next().map(|res| res.map(map_termion_key))
    }
} 