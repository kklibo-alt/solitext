// Terminal module to encapsulate all termion functionality

// For keys and input handling
pub use termion::event::Key;

// For terminal mode
pub use termion::raw::{IntoRawMode, RawTerminal};

// For cursor and terminal control
pub mod cursor {
    pub use termion::cursor::{Goto, Hide, Show};
}

pub mod clear {
    pub use termion::clear::All;
}

// For input handling
pub trait TermReader {
    fn keys(&self) -> termion::input::Keys<std::io::Stdin>;
}

impl TermReader for std::io::Stdin {
    fn keys(&self) -> termion::input::Keys<std::io::Stdin> {
        termion::input::TermRead::keys(self)
    }
}

// Re-export TermRead for additional methods if needed
pub use termion::input::TermRead;

// For colors
pub mod color {
    pub use termion::color::{
        self, Bg, Black, Blue, Color, Fg, Green, LightBlack, LightBlue, LightGreen, LightRed,
        LightWhite, Red, Reset, White,
    };
} 