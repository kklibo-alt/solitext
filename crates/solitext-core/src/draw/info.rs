//! Draws info for the user: text labels, instructions, menus, messages, etc.

use super::Draw;
use crate::game_state::GameState;
use crate::terminal::{TerminalColor, termion_impl::TermionColor};
use std::io::Write;
use std::{thread, time};

impl Draw {
    pub(super) fn display_info(&mut self) {
        let light_yellow = TermionColor::light_yellow();
        let default_bg = self.default_bg();
        self.set_colors(&light_yellow, &default_bg);
        self.draw_text(1, 1, "Solitext");

        let light_black = TermionColor::light_black();
        self.set_colors(&light_black, &default_bg);
        self.draw_text(32, 1, "h: Help  Esc: Menu");
        self.draw_text(2, Self::CURSOR_ROW + 1, "Space: Select/Move cards");
        self.draw_text(
            2,
            Self::CURSOR_ROW + 2,
            self.context_help_message.clone().as_str(),
        );
        if self.debug_mode {
            self.draw_text(2, Self::CURSOR_ROW + 3, self.debug_message.clone().as_str());
        }
    }

    fn display_victory_message(&mut self) {
        const CENTER: (usize, usize) = (26, 5);
        const WIDTH_VAL: usize = 3;
        fn draw_box(s: &mut Draw, size: usize) {
            s.draw_box(
                CENTER.0 - WIDTH_VAL - size,
                CENTER.1 - size,
                CENTER.0 + WIDTH_VAL + size,
                CENTER.1 + size,
            );
        }
        fn pause() {
            thread::sleep(time::Duration::from_millis(300));
        }

        let blue = TermionColor::light_blue();
        let default_bg = self.default_bg();
        self.set_colors(&blue, &default_bg);
        draw_box(self, 3);
        pause();
        
        let green = TermionColor::light_green();
        self.set_colors(&green, &default_bg);
        draw_box(self, 2);
        pause();
        
        let red = TermionColor::red();
        self.set_colors(&red, &default_bg);
        draw_box(self, 1);
        pause();

        let light_yellow = TermionColor::light_yellow();
        let light_blue = TermionColor::light_blue();
        self.set_colors(&light_yellow, &light_blue);
        self.draw_text(CENTER.0 - 3, CENTER.1, "YOU WIN");
        pause();
        pause();
        
        let default_fg = self.default_fg();
        let default_bg = self.default_bg();
        self.set_colors(&default_fg, &default_bg);
        self.draw_text(CENTER.0 - 8, CENTER.1 + 4, "Play again? (y/n)");
    }

    pub fn display_victory(&mut self, game_state: &mut GameState) {
        self.clear_screen();
        //just display cards
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        self.display_victory_message();

        let default_fg = self.default_fg();
        let default_bg = self.default_bg();
        self.set_colors(&default_fg, &default_bg);
        self.terminal.flush().unwrap();
    }

    pub fn display_start_screen(&mut self) {
        self.clear_screen();
        
        let light_yellow = TermionColor::light_yellow();
        let default_bg = self.default_bg();
        self.set_colors(&light_yellow, &default_bg);
        self.draw_text(16, 1, "Solitext    ♥ ♠ ♦ ♣");

        let lines = r#"1: New Game (Draw One)
3: New Game (Draw Three)
Esc: Quit"#;
        self.draw_text_box(lines);

        let default_fg = self.default_fg();
        self.set_colors(&default_fg, &default_bg);
        self.terminal.flush().unwrap();
    }

    pub fn display_game_menu(&mut self, game_state: &mut GameState) {
        self.clear_screen();
        //just display cards
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        let lines = r#"1: New Game (Draw One)
3: New Game (Draw Three)
r: Restart current game
q: Quit
Esc: Return to game"#;
        self.draw_text_box(lines);

        let default_fg = self.default_fg();
        let default_bg = self.default_bg();
        self.set_colors(&default_fg, &default_bg);
        self.terminal.flush().unwrap();
    }

    pub fn display_help(&mut self, game_state: &mut GameState) {
        self.clear_screen();
        //just display cards
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        let lines = r#"Controls:

 Arrow keys, Home, End: Move cursor
 Enter: Hit/move card to stack
 Space: Select/move cards
 x: Clear selection
 Ctrl+c: Quit"#;
        self.draw_text_box(lines);

        let default_fg = self.default_fg();
        let default_bg = self.default_bg();
        self.set_colors(&default_fg, &default_bg);
        self.terminal.flush().unwrap();
    }

    pub(super) fn display_info_section(&mut self) {
        let black = TermionColor::black();
        let yellow = TermionColor::yellow();
        let light_yellow = TermionColor::light_yellow();
        let white = TermionColor::white();
        let light_black = TermionColor::light_black();
        
        self.set_colors(&light_yellow, &black);
        self.draw_text(2, 20, "[space] - select card");
        
        self.set_colors(&yellow, &black);
        self.draw_text(2, 21, "[enter] - confirm movement");
        
        self.set_colors(&white, &black);
        self.draw_text(2, 22, "[esc]   - cancel");
        
        self.set_colors(&light_black, &black);
        self.draw_text(2, 24, "quit: [ctrl+c]");
    }
}
