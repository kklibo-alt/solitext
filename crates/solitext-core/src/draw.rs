mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::selection::Selection;
use crate::terminal::Terminal;
use std::io::{Write};

// Generic Draw struct that works with any terminal implementation
pub struct Draw<T>
where 
    T: Terminal + Write,
    T::RawTerminal: Write,
{
    pub stdout: T::RawTerminal,
    pub cursor: Selection,
    pub selected: Option<Selection>,
    pub context_help_message: String,
    pub debug_message: String,
    pub debug_mode: bool,
}

impl<T> Draw<T> 
where 
    T: Terminal + Write,
    T::RawTerminal: Write,
{
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
