//! Draws the full game state with selections.

use super::Draw;
use crate::game_state::GameState;
use crate::selection::Selection;
use crate::terminal::{TerminalColor, termion_impl::TermionColor};

impl Draw {
    pub fn display_game_state(&mut self, game_state: &GameState) {
        self.clear_screen();
        self.set_colors(&self.default_fg(), &self.default_bg());

        self.display_info();
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        let blue = TermionColor::light_blue();
        self.set_colors(&blue, &self.default_bg());
        self.display_collection_selection_cursor();

        let green_bg = TermionColor::light_green();
        self.set_colors(&self.default_fg(), &green_bg);
        self.display_card_selection_cursor(self.cursor, game_state);

        let yellow_bg = TermionColor::white();
        self.set_colors(&self.default_fg(), &yellow_bg);
        if let Some(selected) = self.selected {
            self.display_card_selection_cursor(selected, game_state);
        }

        self.set_colors(&self.default_fg(), &self.default_bg());
    }

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
