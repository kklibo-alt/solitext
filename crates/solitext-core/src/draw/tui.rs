//! Drawing module for a terminal UI

use super::{common::DEFAULT_DRAW_AREA, Draw};
use crate::game_state::GameState;
use crate::selection::Selection;
use crate::terminal::{TerminalColor, termion_impl::TermionColor};

impl Draw {
    pub fn refresh_gamestate(&mut self, game_state: &GameState) {
        // clear
        self.set_up_terminal();

        // draw Tableau
        self.display_columns(game_state);

        // draw Stock
        self.display_stock_deck(game_state);

        // draw Waste
        self.display_waste(game_state);

        // draw Foundations
        self.display_foundations(game_state);

        // highlight cards from selection
        match self.cursor {
            Selection::Column { index, card_count } => {
                let col = Self::COLUMNS_INIT_COL + Self::COLUMNS_COL_STEP * index;
                self.draw_card_column_selection_cursor(game_state, col, index, card_count);
            }
            Selection::Foundation { index } => {
                let col = Self::FOUNDATIONS_INIT_COL + Self::FOUNDATIONS_COL_STEP * index;
                self.draw_cell_selection(col, Self::FOUNDATIONS_INIT_ROW);
            }
            Selection::Waste => {
                if let Some((_card, _card_state)) = game_state.waste.top_card() {
                    self.draw_cell_selection(Self::WASTE_INIT_COL, Self::WASTE_INIT_ROW);
                }
            }
            Selection::Stock => {
                self.draw_cell_selection(Self::STOCK_INIT_COL, Self::STOCK_INIT_ROW);
            }
        }

        // draw movement target highlight
        if let Some(selection) = self.selected {
            match selection {
                Selection::Column { index, card_count: _ } => {
                    let color = TermionColor::light_blue();
                    let white = TermionColor::white();
                    self.set_colors(&color, &white);
                    let col = Self::COLUMNS_INIT_COL + Self::COLUMNS_COL_STEP * index;
                    self.draw_text(col, Self::COLUMNS_INIT_ROW - 1, "^^^");
                }
                Selection::Foundation { index } => {
                    let color = TermionColor::light_blue();
                    let white = TermionColor::white();
                    self.set_colors(&color, &white);
                    let col = Self::FOUNDATIONS_INIT_COL + Self::FOUNDATIONS_COL_STEP * index;
                    self.draw_text(col, Self::FOUNDATIONS_INIT_ROW - 1, "^^^");
                }
                Selection::Waste => {
                    let color = TermionColor::light_blue();
                    let white = TermionColor::white();
                    self.set_colors(&color, &white);
                    self.draw_text(Self::WASTE_INIT_COL, Self::WASTE_INIT_ROW - 1, "^^^");
                }
                Selection::Stock => {
                    let color = TermionColor::light_blue();
                    let white = TermionColor::white();
                    self.set_colors(&color, &white);
                    self.draw_text(Self::STOCK_INIT_COL, Self::STOCK_INIT_ROW - 1, "^^^");
                }
            }
        }

        // Draw info and legend
        self.display_footer();

        // flush to the terminal
        self.terminal.flush().unwrap();
    }

    pub(super) fn draw_cell_selection(&mut self, col: usize, row: usize) {
        let default_fg = self.default_fg();
        let default_bg = self.default_bg();

        self.set_colors(&default_fg, &default_bg);
        self.draw_text(col - 1, row, "[");
        self.draw_text(col + 3, row, "]");
    }

    pub fn set_up_terminal(&mut self) {
        self.terminal.clear().unwrap();
        self.terminal.goto(1, 1).unwrap();

        // Display title.
        let light_yellow = TermionColor::light_yellow();
        let default_bg = self.default_bg();
        self.set_colors(&light_yellow, &default_bg);
        self.draw_text(1, 1, "Solitext");

        let light_black = TermionColor::light_black();
        self.set_colors(&light_black, &default_bg);
        self.draw_text(32, 1, "h: Help  Esc: Menu");
        self.draw_text(
            DEFAULT_DRAW_AREA.width - 15,
            DEFAULT_DRAW_AREA.height,
            "ctrl+c to quit",
        );
    }
} 