mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

use crate::selection::Selection;
use ratatui::buffer::Buffer;
use ratatui::style::Style;
use ratatui::DefaultTerminal;

pub struct Draw {
    terminal: DefaultTerminal,
    restored: bool,
    pub cursor: Selection,
    pub selected: Option<Selection>,
    pub context_help_message: String,
    pub debug_message: String,
    pub debug_mode: bool,
}

pub(crate) struct Renderer<'a> {
    buf: &'a mut Buffer,
    style: Style,
    pub(super) cursor: Selection,
    pub(super) selected: Option<Selection>,
    pub(super) debug_mode: bool,
}

impl<'a> Renderer<'a> {
    pub(crate) fn new(
        buf: &'a mut Buffer,
        cursor: Selection,
        selected: Option<Selection>,
        debug_mode: bool,
    ) -> Self {
        Self {
            buf,
            style: Style::default().fg(Self::default_fg()).bg(Self::default_bg()),
            cursor,
            selected,
            debug_mode,
        }
    }

    pub(crate) fn clear(&mut self) {
        let area = *self.buf.area();
        self.buf
            .set_style(area, Style::default().fg(Self::default_fg()).bg(Self::default_bg()));
    }
}

impl Draw {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            restored: false,
            cursor: Selection::Deck,
            selected: None,
            context_help_message: String::new(),
            debug_message: String::new(),
            debug_mode: false,
        }
    }

    pub fn restore(&mut self) {
        if !self.restored {
            ratatui::restore();
            self.restored = true;
        }
    }
}

impl Drop for Draw {
    fn drop(&mut self) {
        self.restore();
    }
}
