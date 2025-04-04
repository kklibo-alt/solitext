//! Draws the 'foundations' (the ascending card piles needed for victory).

use super::Draw;
use crate::cards::Suit;
use crate::game_state::{CardState, GameState};
use crate::terminal::{TerminalColor, termion_impl::TermionColor};

impl Draw {
    pub(super) fn draw_pile_selection_cursor(&mut self, col: usize, index: usize) {
        let row = Self::PILES_INIT_ROW + Self::PILES_ROW_STEP * index;
        self.draw_text(col - 1, row, "[");
        self.draw_text(col + 3, row, "]");
    }

    pub(super) const PILES_INIT_COL: usize = 48;
    const PILES_INIT_ROW: usize = 2;
    const PILES_ROW_STEP: usize = 2;
    pub(super) fn display_piles(&mut self, game_state: &GameState) {
        let (init_col, init_row) = (Self::PILES_INIT_COL, Self::PILES_INIT_ROW);
        let mut row = init_row;
        for (index, pile) in game_state.card_piles.iter().enumerate() {
            if let Some(card) = pile.0.last() {
                self.display_card(*card, CardState::FaceUp, init_col, row);
            } else {
                let blue = TermionColor::light_blue();
                let light_black = TermionColor::light_black();
                self.set_colors(&blue, &light_black);
                self.draw_text(
                    init_col,
                    row,
                    format!(
                        "{}_",
                        Suit::from_index(index).expect("pile suit should exist")
                    )
                    .as_str(),
                );
            };

            row += Self::PILES_ROW_STEP;
        }
    }

    pub(super) const FOUNDATIONS_INIT_COL: usize = 28;
    pub(super) const FOUNDATIONS_INIT_ROW: usize = 2;
    pub(super) const FOUNDATIONS_COL_STEP: usize = 5;

    pub(super) fn display_foundations(&mut self, game_state: &GameState) {
        let mut col = Self::FOUNDATIONS_INIT_COL;
        for i in 0..4 {
            let pile = &game_state.card_piles[i];
            if let Some(card) = pile.0.last() {
                self.display_card(*card, CardState::FaceUp, col, Self::FOUNDATIONS_INIT_ROW);
            } else {
                let empty_char = match i {
                    0 => "♥",
                    1 => "♠",
                    2 => "♦",
                    3 => "♣",
                    _ => unreachable!(),
                };
                let blue = TermionColor::light_blue();
                let light_black = TermionColor::light_black();
                self.set_colors(&blue, &light_black);
                self.draw_text(col, Self::FOUNDATIONS_INIT_ROW, &format!(" {} ", empty_char));
            }
            col += Self::FOUNDATIONS_COL_STEP;
        }
    }
}
