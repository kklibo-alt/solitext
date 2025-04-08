mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::{selection::Selection, terminal::TermionTerminal};
use std::io::{Stdout, stdout};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Draw {
    pub stdout: RawTerminal<Stdout>,
    pub terminal: TermionTerminal,
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
            stdout: stdout().into_raw_mode().unwrap(),
            terminal: TermionTerminal::new(),
            cursor: Selection::Deck,
            selected: None,
            context_help_message: "".to_string(),
            debug_message: "".to_string(),
            debug_mode: false,
        }
    }
}
