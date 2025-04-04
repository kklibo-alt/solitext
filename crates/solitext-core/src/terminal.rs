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
pub use termion::input::Keys;
pub use termion::input::TermRead;

// Simple wrapper to make the code cleaner when refactoring
pub fn stdin_keys() -> Keys<std::io::Stdin> {
    std::io::stdin().keys()
}

// For colors
pub mod color {
    pub use termion::color::{
        self, Bg, Black, Blue, Color, Fg, Green, LightBlack, LightBlue, LightGreen, LightRed,
        LightWhite, Red, Reset, White, Yellow,
    };
    
    // Use Yellow for LightYellow for now
    pub use Yellow as LightYellow;
} 