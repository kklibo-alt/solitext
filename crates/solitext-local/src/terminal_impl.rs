use std::io::{Write, stdin, stdout};
use termion;
use termion::input::TermRead;
use solitext_core::terminal::Key;
use solitext_core::terminal::adapters::ColorProvider;
use solitext_core::terminal::adapters::TerminalProvider;

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

// Define our own color types to implement ColorProvider
pub struct LocalBlack;
pub struct LocalRed;
pub struct LocalGreen;
pub struct LocalYellow;
pub struct LocalBlue;
pub struct LocalMagenta;
pub struct LocalCyan;
pub struct LocalWhite;
pub struct LocalLightBlack;
pub struct LocalLightRed;
pub struct LocalLightGreen;
pub struct LocalLightYellow;
pub struct LocalLightBlue;
pub struct LocalLightMagenta;
pub struct LocalLightCyan;
pub struct LocalLightWhite;
pub struct LocalReset;

// Implement ColorProvider for each color type
impl ColorProvider for LocalBlack {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Black))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Black))
    }
}

impl ColorProvider for LocalRed {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Red))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Red))
    }
}

impl ColorProvider for LocalGreen {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Green))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Green))
    }
}

impl ColorProvider for LocalYellow {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Yellow))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Yellow))
    }
}

impl ColorProvider for LocalBlue {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Blue))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Blue))
    }
}

impl ColorProvider for LocalMagenta {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Magenta))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Magenta))
    }
}

impl ColorProvider for LocalCyan {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Cyan))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Cyan))
    }
}

impl ColorProvider for LocalWhite {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::White))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::White))
    }
}

impl ColorProvider for LocalLightBlack {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightBlack))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightBlack))
    }
}

impl ColorProvider for LocalLightRed {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightRed))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightRed))
    }
}

impl ColorProvider for LocalLightGreen {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightGreen))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightGreen))
    }
}

impl ColorProvider for LocalLightYellow {
    fn fg_code(&self) -> String {
        // Use Yellow since there's no LightYellow in termion
        format!("{}", termion::color::Fg(termion::color::Yellow))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Yellow))
    }
}

impl ColorProvider for LocalLightBlue {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightBlue))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightBlue))
    }
}

impl ColorProvider for LocalLightMagenta {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightMagenta))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightMagenta))
    }
}

impl ColorProvider for LocalLightCyan {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightCyan))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightCyan))
    }
}

impl ColorProvider for LocalLightWhite {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::LightWhite))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::LightWhite))
    }
}

impl ColorProvider for LocalReset {
    fn fg_code(&self) -> String {
        format!("{}", termion::color::Fg(termion::color::Reset))
    }
    fn bg_code(&self) -> String {
        format!("{}", termion::color::Bg(termion::color::Reset))
    }
}

// Define our own terminal type
pub struct LocalTerminal {
    stdout: std::io::Stdout,
}

impl Default for LocalTerminal {
    fn default() -> Self {
        Self { stdout: stdout() }
    }
}

impl Write for LocalTerminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

// Implement TerminalProvider for LocalTerminal
impl TerminalProvider for LocalTerminal {
    type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;
    
    fn into_raw_mode(self) -> std::io::Result<Self::RawTerminal> {
        termion::raw::IntoRawMode::into_raw_mode(self.stdout)
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
#[derive(Default)]
pub struct Stdin;

impl Stdin {
    pub fn new() -> Self {
        Stdin
    }
}

impl solitext_core::terminal::TerminalInput for Stdin {
    type Keys = termion::input::Keys<std::io::Stdin>;
    
    fn keys(self) -> Self::Keys {
        stdin().keys()
    }
    
    fn read_key(keys: &mut Self::Keys) -> Option<std::io::Result<Key>> {
        keys.next().map(|res| res.map(map_termion_key))
    }
} 