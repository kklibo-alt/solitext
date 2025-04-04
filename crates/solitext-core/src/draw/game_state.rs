//! Draws the full game state with selections.

use super::Draw;
use crate::game_state::GameState;
use crate::selection::Selection;
use crate::terminal::{Blue, Color, LightGreen, LightYellow, Terminal};
use std::io::Write;

impl<T> Draw<T>
where
    T: Terminal + Write,
    T::RawTerminal: Write,
{
    pub fn display_game_state<BG: Color + Clone, FG: Color + Clone, ACB: Color + Clone, ACC1: Color + Clone, ACC2: Color + Clone>(
        &mut self,
        game_state: &GameState,
        fg: FG,
        bg: BG,
        accent_bg: ACB,
        accent_cursor1: ACC1,
        accent_cursor2: ACC2,
    ) {
        self.clear_screen();
        self.set_colors(fg.clone(), bg.clone());

        self.display_info(fg.clone(), bg.clone());
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        self.set_colors(accent_bg.clone(), bg.clone());
        self.display_collection_selection_cursor();

        self.set_colors(fg.clone(), accent_cursor1);
        self.display_card_selection_cursor(self.cursor, game_state);

        self.set_colors(fg.clone(), accent_cursor2);
        if let Some(selected) = self.selected {
            self.display_card_selection_cursor(selected, game_state);
        }

        self.set_colors(fg, bg);
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
