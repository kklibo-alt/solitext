use super::Renderer;
use crate::cards::Card;
use crate::game_state::CardState;
use ratatui::style::Color;

impl Renderer<'_> {
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
                    self.set_colors(Color::Red, Color::Gray);
                } else {
                    self.set_colors(Color::Black, Color::Gray);
                }
                card.to_string()
            }
            CardState::FaceDown => {
                if self.debug_mode {
                    if card.suit.is_red() {
                        self.set_colors(Color::LightRed, Color::Black);
                    } else {
                        self.set_colors(Color::DarkGray, Color::Black);
                    }
                    card.to_string()
                } else {
                    self.set_colors(Color::LightGreen, Color::DarkGray);
                    "st".to_string()
                }
            }
        };

        self.draw_text(col, row, text.as_str());
    }
}
