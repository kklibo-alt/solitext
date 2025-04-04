//! Draws info for the user: text labels, instructions, menus, messages, etc.

use super::Draw;
use crate::game_state::GameState;
use crate::terminal::TerminalColor;
use std::io::Write;
use std::{thread, time};

impl Draw {
    pub(super) fn display_info(&mut self) {
        self.set_colors(TerminalColor::LightYellow, Self::default_bg());
        self.draw_text(1, 1, "Solitext");

        self.set_colors(TerminalColor::LightBlack, Self::default_bg());
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

        self.set_colors(TerminalColor::Blue, Self::default_bg());
        draw_box(self, 3);
        pause();
        self.set_colors(TerminalColor::Green, Self::default_bg());
        draw_box(self, 2);
        pause();
        self.set_colors(TerminalColor::Red, Self::default_bg());
        draw_box(self, 1);
        pause();

        self.set_colors(TerminalColor::LightYellow, TerminalColor::LightBlue);
        self.draw_text(CENTER.0 - 3, CENTER.1, "YOU WIN");
        pause();
        pause();
        self.set_colors(Self::default_fg(), Self::default_bg());
        self.draw_text(CENTER.0 - 8, CENTER.1 + 4, "Play again? (1/3/q)");
    }

    pub fn display_victory(&mut self, game_state: &mut GameState) {
        self.clear_screen();
        //just display cards
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        self.display_victory_message();

        self.set_colors(Self::default_fg(), Self::default_bg());
        self.terminal.flush().unwrap();
    }

    pub fn display_start_screen(&mut self) {
        self.clear_screen();
        self.set_colors(TerminalColor::LightYellow, Self::default_bg());
        self.draw_text(16, 1, "Solitext    ♥ ♠ ♦ ♣");

        let lines = r#"1: New Game (Draw One)
3: New Game (Draw Three)
Esc: Quit"#;
        self.draw_text_box(lines);

        self.set_colors(Self::default_fg(), Self::default_bg());
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

        self.set_colors(Self::default_fg(), Self::default_bg());
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

        self.set_colors(Self::default_fg(), Self::default_bg());
        self.terminal.flush().unwrap();
    }
}
