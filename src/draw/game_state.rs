use super::{Draw, Renderer};
use crate::game_state::GameState;
use crate::selection::Selection;
use ratatui::style::Color;

impl Draw {
    pub fn display_game_state(&mut self, game_state: &GameState) {
        let cursor = self.cursor;
        let selected = self.selected;
        let debug_mode = self.debug_mode;
        let debug_msg = self.debug_message.clone();
        let ctx_msg = self.context_help_message.clone();

        self.terminal
            .draw(|frame| {
                let buf = frame.buffer_mut();
                let mut r = Renderer::new(buf, cursor, selected, debug_mode);
                r.clear();

                r.display_info(&ctx_msg, &debug_msg);
                r.display_deck(game_state);
                r.display_columns(game_state);
                r.display_piles(game_state);

                r.set_colors(Color::Blue, Renderer::default_bg());
                r.display_collection_selection_cursor();

                r.set_colors(Renderer::default_fg(), Color::LightGreen);
                r.display_card_selection_cursor(cursor, game_state);

                r.set_colors(Renderer::default_fg(), Color::LightYellow);
                if let Some(selected) = selected {
                    r.display_card_selection_cursor(selected, game_state);
                }

                r.set_colors(Renderer::default_fg(), Renderer::default_bg());
            })
            .unwrap();
    }
}

impl Renderer<'_> {
    fn selection_col(selection: Selection) -> usize {
        match selection {
            Selection::Deck => Self::DECK_INIT_COL,
            Selection::Column { index, .. } => {
                Self::COLUMNS_INIT_COL + index * Self::COLUMNS_COL_STEP
            }
            Selection::Pile { .. } => Self::PILES_INIT_COL,
        }
    }

    pub(super) const CURSOR_ROW: usize = 10;
    fn display_collection_selection_cursor(&mut self) {
        let col = Self::selection_col(self.cursor);
        self.draw_text(col, Self::CURSOR_ROW, "█↑█");
    }

    fn display_card_selection_cursor(&mut self, selection: Selection, game_state: &GameState) {
        let col = Self::selection_col(selection);

        match selection {
            Selection::Deck => {
                if let Some(row) = Self::deck_selection_cursor_row(game_state) {
                    self.draw_deck_selection_cursor(col, row);
                }
            }
            Selection::Column { index, card_count } => {
                self.draw_card_column_selection_cursor(game_state, col, index, card_count)
            }
            Selection::Pile { index } => self.draw_pile_selection_cursor(col, index),
        };
    }
}
