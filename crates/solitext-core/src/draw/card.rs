//! Draws individual cards with their suits and ranks.

use super::Draw;
use crate::cards::{Card, Rank, Suit};
use crate::game_state::CardState;
use crate::terminal::{Terminal, Red, White, LightBlack, LightRed, Black};
use std::io::Write;

impl<T> Draw<T> 
where 
    T: Terminal + Write,
    T::RawTerminal: Write,
{
    fn card_display_repr(card: Card) -> String {
        let suit_repr = match card.suit {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
        };

        let rank_repr = match card.rank {
            Rank::Ace => "A ",
            Rank::R2 => "2 ",
            Rank::R3 => "3 ",
            Rank::R4 => "4 ",
            Rank::R5 => "5 ",
            Rank::R6 => "6 ",
            Rank::R7 => "7 ",
            Rank::R8 => "8 ",
            Rank::R9 => "9 ",
            Rank::R10 => "10",
            Rank::Jack => "J ",
            Rank::Queen => "Q ",
            Rank::King => "K ",
        };

        if rank_repr.len() > 1 {
            format!("{}{}", rank_repr, suit_repr)
        } else {
            format!("{} {}", rank_repr, suit_repr)
        }
    }

    fn card_is_red(card: Card) -> bool {
        matches!(card.suit, Suit::Hearts | Suit::Diamonds)
    }

    pub(super) fn display_card(&mut self, card: Card, face: CardState, col: usize, row: usize) {
        let rank_repr = Self::card_display_repr(card);

        match face {
            CardState::FaceUp => {
                if Self::card_is_red(card) {
                    self.set_colors(LightRed, White);
                } else {
                    self.set_colors(Black, White);
                }
                self.draw_text(col, row, rank_repr.as_str());
            }
            CardState::FaceDown => {
                self.set_colors(Red, LightBlack);
                self.draw_text(col, row, "###");
            }
        }
    }
}
