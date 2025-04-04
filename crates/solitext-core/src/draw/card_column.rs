//! Draws a column of cards where each card is offset from the card below it.

use super::Draw;
use crate::game_state::{CardState, GameState};
use crate::terminal::Terminal;
use std::io::Write;

impl<T> Draw<T> 
where 
    T: Terminal + Write,
    T::RawTerminal: Write,
{
    pub(super) fn draw_card_column_selection_cursor(
        &mut self,
        game_state: &GameState,
        col: usize,
        idx: usize,
        card_count: usize,
    ) {
        let col_cards = &game_state.columns[idx].0;
        if col_cards.is_empty() || card_count == 0 {
            return;
        }

        let row = Self::COLUMNS_INIT_ROW + col_cards.len() - card_count;
        self.draw_text(col, row, "◄");
        self.draw_text(col + 4, row, "►");
    }

    pub(super) const COLUMNS_INIT_ROW: usize = 3;
    pub(super) const COLUMNS_COL_STEP: usize = 8;
    pub(super) const COLUMNS_INIT_COL: usize = 10;
    pub(super) fn display_columns(&mut self, game_state: &GameState) {
        for (idx, col) in game_state.columns.iter().enumerate() {
            let (col_x, mut col_y) = (
                Self::COLUMNS_INIT_COL + idx * Self::COLUMNS_COL_STEP,
                Self::COLUMNS_INIT_ROW,
            );

            if col.0.is_empty() {
                continue;
            }

            for &(card, card_state) in col.0.iter() {
                self.display_card(card, card_state, col_x, col_y);
                col_y += 1;
            }
        }
    }
}
