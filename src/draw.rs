mod card;
mod card_column;
mod common;
mod deck;
mod foundation;
mod game_state;
mod info;

#[cfg(feature = "web")]
pub(crate) use game_state::render_game;
#[cfg(feature = "web")]
pub(crate) use info::{render_game_menu, render_help, render_start_screen, render_victory};

use crate::selection::Selection;
use ratatui::buffer::Buffer;
use ratatui::style::Style;
#[cfg(feature = "native")]
use ratatui::DefaultTerminal;

#[cfg(feature = "native")]
pub struct Draw {
    terminal: DefaultTerminal,
    restored: bool,
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

#[cfg(feature = "native")]
impl Draw {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            restored: false,
        }
    }

    pub fn restore(&mut self) {
        if !self.restored {
            ratatui::restore();
            self.restored = true;
        }
    }
}

#[cfg(feature = "native")]
impl Drop for Draw {
    fn drop(&mut self) {
        self.restore();
    }
}
