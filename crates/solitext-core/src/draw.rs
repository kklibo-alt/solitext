mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::{selection::Selection, terminal::Color, terminal::Terminal};
use std::io::{Stdout, stdout};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Draw<T: Terminal> {
    pub stdout: RawTerminal<Stdout>,
    pub terminal: T,
    pub cursor: Selection,
    pub selected: Option<Selection>,
    pub context_help_message: String,
    pub debug_message: String,
    pub debug_mode: bool,
}

impl<T: Terminal> Draw<T> {
    pub fn new(terminal: T) -> Self {
        Self {
            stdout: stdout().into_raw_mode().unwrap(),
            terminal,
            cursor: Selection::Deck,
            selected: None,
            context_help_message: "".to_string(),
            debug_message: "".to_string(),
            debug_mode: false,
        }
    }

    pub fn set_colors(&mut self, foreground: Color, background: Color) {
        self.terminal.set_colors2(foreground, background);
    }
}
