//! Common drawing code.

use super::Draw;
use crate::terminal;
use crate::terminal::Terminal;
use std::io::Write;
use termion::clear;

impl<T: Terminal> Draw<T> {
    pub(crate) fn clear_screen(&mut self) {
        writeln!(self.stdout, "{}", clear::All,).unwrap();
    }
    pub(crate) fn default_bg2() -> terminal::Color {
        terminal::Color::Black
    }
    pub(crate) fn default_fg2() -> terminal::Color {
        terminal::Color::LightWhite
    }

    pub(crate) fn draw_box(&mut self, col1: usize, row1: usize, col2: usize, row2: usize) {
        use std::cmp::{max, min};
        for col in min(col1, col2)..=max(col1, col2) {
            for row in min(row1, row2)..=max(row1, row2) {
                self.draw_text(col, row, "█");
            }
        }
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
        self.set_colors(terminal::Color::LightBlue, Self::default_bg2());
        self.draw_centered_box(WIDTH, height + 2);
        self.set_colors(terminal::Color::White, Self::default_bg2());
        self.draw_centered_box(WIDTH - 2, height);

        self.set_colors(terminal::Color::LightBlack, terminal::Color::White);
        let (col, mut row, _, _) = Self::centered_box_corners(WIDTH - 2, height);

        for line in lines.split('\n') {
            self.draw_text(col, row, line);
            row += 1;
        }
    }
}
