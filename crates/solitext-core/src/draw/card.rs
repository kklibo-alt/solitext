//! Draws a card.

use super::Draw;
use crate::cards::Card;
use crate::game_state::CardState;
use crate::terminal::{termion_impl::TermionColor, TerminalColor};

impl Draw {
    pub(crate) fn display_card(
        &mut self,
        card: Card,
        card_state: CardState,
        col: usize,
        row: usize,
    ) {
        let text = match card_state {
            CardState::FaceUp => {
                if card.suit.is_red() {
                    let red = TermionColor::red();
                    let white = TermionColor::white();
                    self.set_colors(&red, &white);
                } else {
                    let black = TermionColor::black();
                    let white = TermionColor::white();
                    self.set_colors(&black, &white);
                }
                card.to_string()
            }
            CardState::FaceDown => {
                if self.debug_mode {
                    if card.suit.is_red() {
                        let light_red = TermionColor::light_red();
                        let black = TermionColor::black();
                        self.set_colors(&light_red, &black);
                    } else {
                        let light_black = TermionColor::light_black();
                        let black = TermionColor::black();
                        self.set_colors(&light_black, &black);
                    }
                    card.to_string()
                } else {
                    let light_green = TermionColor::light_green();
                    let light_black = TermionColor::light_black();
                    self.set_colors(&light_green, &light_black);
                    "st".to_string()
                }
            }
        };

        self.draw_text(col, row, text.as_str());
    }
}
