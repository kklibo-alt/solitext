mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::selection::Selection;
use crate::terminal::{Terminal, termion_impl::TermionTerminal};
use std::io::Write;

pub struct Draw {
    pub terminal: Box<dyn Terminal>,
    pub cursor: Selection,
    pub selected: Option<Selection>,
    pub context_help_message: String,
    pub debug_message: String,
    pub debug_mode: bool,
}

impl Default for Draw {
    fn default() -> Self {
        Self::new()
    }
}

impl Draw {
    pub fn new() -> Self {
        Self {
            terminal: Box::new(TermionTerminal::new()),
            cursor: Selection::Deck,
            selected: None,
            context_help_message: "".to_string(),
            debug_message: "".to_string(),
            debug_mode: false,
        }
    }
}
