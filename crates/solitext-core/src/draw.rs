mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::selection::Selection;
use crate::terminal::{Terminal, TerminalFactory};
use std::io::{self, Write};

pub struct Draw {
    terminal: Box<dyn Terminal + 'static>,
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
        unimplemented!("Use Draw::with_terminal_factory instead")
    }
    
    pub fn with_terminal_factory<F: TerminalFactory>(factory: &F) -> io::Result<Self> 
    where
        F::Terminal: 'static
    {
        Ok(Self {
            terminal: Box::new(factory.create_terminal()?),
            cursor: Selection::Deck,
            selected: None,
            context_help_message: "".to_string(),
            debug_message: "".to_string(),
            debug_mode: false,
        })
    }
    
    pub fn get_terminal_mut(&mut self) -> &mut dyn Terminal {
        &mut *self.terminal
    }
}
