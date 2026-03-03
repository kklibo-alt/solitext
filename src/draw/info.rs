use super::Renderer;
use crate::app::App;
use crate::game_state::GameState;
use crate::selection::Selection;
use ratatui::style::Color;
use ratatui::Frame;
#[cfg(feature = "native")]
use std::{thread, time};

impl Renderer<'_> {
    pub(super) fn display_info(&mut self, context_help_message: &str, debug_message: &str) {
        self.set_colors(Color::LightYellow, Self::default_bg());
        self.draw_text(1, 1, "Solitext");

        self.set_colors(Color::DarkGray, Self::default_bg());
        self.draw_text(32, 1, "h: Help  Esc: Menu");
        self.draw_text(2, Self::CURSOR_ROW + 1, "Space: Select/Move cards");
        self.draw_text(2, Self::CURSOR_ROW + 2, context_help_message);
        if self.debug_mode {
            self.draw_text(2, Self::CURSOR_ROW + 3, debug_message);
        }
    }
}

fn render_base_game(r: &mut Renderer, game_state: &GameState) {
    r.display_deck(game_state);
    r.display_columns(game_state);
    r.display_piles(game_state);
}

pub(crate) fn render_start_screen(frame: &mut Frame) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, Selection::Deck, None, false);
    r.clear();

    r.set_colors(Color::LightYellow, Renderer::default_bg());
    r.draw_text(16, 1, "Solitext    ♥ ♠ ♦ ♣");

    let lines = r#"1: New Game (Draw One)
3: New Game (Draw Three)
Esc: Quit"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

pub(crate) fn render_game_menu(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, app.cursor, app.selected, app.debug_mode);
    r.clear();

    render_base_game(&mut r, &app.game_state);

    let lines = r#"1: New Game (Draw One)
3: New Game (Draw Three)
r: Restart current game
q: Quit
Esc: Return to game"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

pub(crate) fn render_help(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, app.cursor, app.selected, app.debug_mode);
    r.clear();

    render_base_game(&mut r, &app.game_state);

    let lines = r#"Controls:

 Arrow keys, Home, End: Move cursor
 Enter: Hit/move card to stack
 Space: Select/move cards
 x: Clear selection
 Ctrl+c: Quit"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

pub(crate) fn render_victory(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, Selection::Deck, None, app.debug_mode);
    r.clear();

    render_base_game(&mut r, &app.game_state);

    const CENTER: (usize, usize) = (26, 5);
    const W: usize = 3;
    let boxes: &[(Color, usize)] = &[
        (Color::Blue, 3),
        (Color::Green, 2),
        (Color::Red, 1),
    ];
    for &(color, size) in boxes {
        r.set_colors(color, Renderer::default_bg());
        r.draw_box(
            CENTER.0 - W - size,
            CENTER.1 - size,
            CENTER.0 + W + size,
            CENTER.1 + size,
        );
    }
    r.set_colors(Color::LightYellow, Color::LightBlue);
    r.draw_text(CENTER.0 - 3, CENTER.1, "YOU WIN");

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
    r.draw_text(CENTER.0 - 8, CENTER.1 + 4, "Play again? (y/n)");
}

#[cfg(feature = "native")]
impl super::Draw {
    pub fn display_victory(&mut self, app: &App) {
        const CENTER: (usize, usize) = (26, 5);
        const W: usize = 3;
        let pause = || thread::sleep(time::Duration::from_millis(300));

        let anim_boxes: &[(Color, usize)] = &[
            (Color::Blue, 3),
            (Color::Green, 2),
            (Color::Red, 1),
        ];

        for step in 0..anim_boxes.len() {
            self.terminal
                .draw(|frame| {
                    let buf = frame.buffer_mut();
                    let mut r = Renderer::new(buf, Selection::Deck, None, app.debug_mode);
                    r.clear();
                    render_base_game(&mut r, &app.game_state);

                    for &(color, size) in &anim_boxes[..=step] {
                        r.set_colors(color, Renderer::default_bg());
                        r.draw_box(
                            CENTER.0 - W - size,
                            CENTER.1 - size,
                            CENTER.0 + W + size,
                            CENTER.1 + size,
                        );
                    }
                })
                .unwrap();
            pause();
        }

        self.terminal
            .draw(|frame| {
                render_victory(frame, app);
            })
            .unwrap();
    }

    pub fn display_start_screen(&mut self) {
        self.terminal
            .draw(|frame| {
                render_start_screen(frame);
            })
            .unwrap();
    }

    pub fn display_game_menu(&mut self, app: &App) {
        self.terminal
            .draw(|frame| {
                render_game_menu(frame, app);
            })
            .unwrap();
    }

    pub fn display_help(&mut self, app: &App) {
        self.terminal
            .draw(|frame| {
                render_help(frame, app);
            })
            .unwrap();
    }
}
