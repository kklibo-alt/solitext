//! Common drawing code.

use super::Draw;
use crate::terminal::TerminalColor;
use std::io::Write;

impl Draw {
    pub(crate) fn clear_screen(&mut self) {
        self.terminal.clear_screen().unwrap();
    }

    pub(crate) fn default_bg() -> TerminalColor {
        TerminalColor::Black
    }
    pub(crate) fn default_fg() -> TerminalColor {
        TerminalColor::LightWhite
    }

    pub(crate) fn set_colors(
        &mut self,
        foreground: TerminalColor,
        background: TerminalColor,
    ) {
        self.terminal.set_fg(foreground).unwrap();
        self.terminal.set_bg(background).unwrap();
    }

    pub(crate) fn draw_box(&mut self, col1: usize, row1: usize, col2: usize, row2: usize) {
        use std::cmp::{max, min};
        for col in min(col1, col2)..=max(col1, col2) {
            for row in min(row1, row2)..=max(row1, row2) {
                self.draw_text(col, row, "█");
            }
        }
    }

    pub fn draw_text(&mut self, col: usize, row: usize, text: &str) {
        let col = u16::try_from(col).expect("column should fit in a u16");
        let row = u16::try_from(row).expect("row should fit in a u16");

        self.terminal.goto(col, row).unwrap();
        write!(self.terminal, "{}", text).unwrap();
    }

    pub fn set_up_terminal(&mut self) {
        self.terminal.set_fg(Self::default_fg()).unwrap();
        self.terminal.set_bg(Self::default_bg()).unwrap();
        self.terminal.clear_screen().unwrap();
        self.terminal.goto(1, 1).unwrap();
        self.terminal.hide_cursor().unwrap();
        self.terminal.flush().unwrap();
    }

    pub fn restore_terminal(&mut self) {
        self.terminal.reset_fg().unwrap();
        self.terminal.reset_bg().unwrap();
        self.terminal.clear_screen().unwrap();
        self.terminal.goto(1, 1).unwrap();
        self.terminal.show_cursor().unwrap();
        self.terminal.flush().unwrap();
    }

    fn centered_box_corners(width: usize, height: usize) -> (usize, usize, usize, usize) {
        const CENTER: (usize, usize) = (26, 5);
        (
            CENTER.0 - width / 2,
            CENTER.1 - height / 2,
            CENTER.0 + width / 2,
            CENTER.1 + height / 2,
        )
    }

    fn draw_centered_box(&mut self, width: usize, height: usize) {
        let (col1, row1, col2, row2) = Self::centered_box_corners(width, height);
        self.draw_box(col1, row1, col2, row2);
    }

    pub fn draw_text_box(&mut self, lines: &str) {
        let height = lines.split('\n').count();

        const WIDTH: usize = 38;
        self.set_colors(TerminalColor::LightBlue, Self::default_bg());
        self.draw_centered_box(WIDTH, height + 2);
        self.set_colors(TerminalColor::White, Self::default_bg());
        self.draw_centered_box(WIDTH - 2, height);

        self.set_colors(TerminalColor::LightBlack, TerminalColor::White);
        let (col, mut row, _, _) = Self::centered_box_corners(WIDTH - 2, height);

        for line in lines.split('\n') {
            self.draw_text(col, row, line);
            row += 1;
        }
    }
}
