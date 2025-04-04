//! Draws info for the user: text labels, instructions, menus, messages, etc.

use super::Draw;
use crate::game_state::GameState;
use crate::terminal::{Color, Terminal};
use std::io::Write;
use std::{thread, time};

impl<T> Draw<T> 
where 
    T: Terminal + Write,
    T::RawTerminal: Write,
{
    pub(super) fn display_info<F: Color + Clone, B: Color + Clone>(&mut self, fg: F, bg: B) {
        self.set_colors(fg, bg);
        self.draw_text(1, 1, "Solitext");

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

    fn display_victory_message<F1: Color + Clone, B1: Color + Clone, B2: Color + Clone>(&mut self, fg: F1, bg: B1, accent_bg: B2) {
        const CENTER: (usize, usize) = (26, 5);
        const WIDTH_VAL: usize = 3;
        fn draw_box<F: Color + Clone, B: Color + Clone, T: Terminal + Write>(s: &mut Draw<T>, size: usize, fg: F, bg: B) 
        where 
            T::RawTerminal: Write,
        {
            s.set_colors(fg, bg);
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

        draw_box(self, 3, fg.clone(), bg.clone());
        pause();
        draw_box(self, 2, fg.clone(), bg.clone());
        pause();
        draw_box(self, 1, fg.clone(), bg.clone());
        pause();

        self.set_colors(fg.clone(), accent_bg);
        self.draw_text(CENTER.0 - 3, CENTER.1, "YOU WIN");
        pause();
        pause();
        self.set_colors(fg, bg);
        self.draw_text(CENTER.0 - 8, CENTER.1 + 4, "Play again? (y/n)");
    }

    pub fn display_victory<F: Color + Clone, B: Color + Clone, AB: Color + Clone>(&mut self, game_state: &mut GameState, fg: F, bg: B, accent_bg: AB) {
        self.clear_screen();
        //just display cards
        self.display_deck(game_state);
        self.display_columns(game_state);
        self.display_piles(game_state);

        self.display_victory_message(fg.clone(), bg.clone(), accent_bg);

        self.set_colors(fg, bg);
        self.stdout.flush().unwrap();
    }

    pub fn display_start_screen<F: Color + Clone, B: Color + Clone, AB: Color + Clone>(&mut self, fg: F, bg: B, accent_bg: AB) {
        self.clear_screen();
        self.set_colors(fg.clone(), bg.clone());
        self.draw_text(16, 1, "Solitext    ♥ ♠ ♦ ♣");

        let lines = r#"1: New Game (Draw One)
3: New Game (Draw Three)
Esc: Quit"#;
        self.draw_text_box(lines, fg.clone(), bg.clone(), fg.clone(), accent_bg);

        self.set_colors(fg, bg);
        self.stdout.flush().unwrap();
    }

    pub fn display_game_menu<F: Color + Clone, B: Color + Clone, AB: Color + Clone>(&mut self, game_state: &mut GameState, fg: F, bg: B, accent_bg: AB) {
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
        self.draw_text_box(lines, fg.clone(), bg.clone(), fg.clone(), accent_bg);

        self.set_colors(fg, bg);
        self.stdout.flush().unwrap();
    }

    pub fn display_help<F: Color + Clone, B: Color + Clone, AB: Color + Clone>(&mut self, game_state: &mut GameState, fg: F, bg: B, accent_bg: AB) {
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
        self.draw_text_box(lines, fg.clone(), bg.clone(), fg.clone(), accent_bg);

        self.set_colors(fg, bg);
        self.stdout.flush().unwrap();
    }
}
