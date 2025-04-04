mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::selection::Selection;
use std::io::{stdout, Write};
use crate::terminal::Terminal;

// Generic Draw struct that can work with any terminal implementation
pub struct Draw<T: Terminal> {
    pub stdout: T::RawTerminal,
    pub cursor: Selection,
    pub selected: Option<Selection>,
    pub context_help_message: String,
    pub debug_message: String,
    pub debug_mode: bool,
}

impl<T: Terminal> Draw<T> {
    pub fn new(terminal: T) -> Self {
        Self {
            stdout: terminal.into_raw_mode().expect("Failed to enter raw mode"),
            cursor: Selection::Deck,
            selected: None,
            context_help_message: "".to_string(),
            debug_message: "".to_string(),
            debug_mode: false,
        }
    }
}
