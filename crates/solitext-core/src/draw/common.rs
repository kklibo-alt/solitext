//! Common drawing code.

use super::Draw;
use crate::terminal::{Black, Color, LightBlack, LightBlue, LightWhite, Reset, Terminal, White};
use std::io::Write;

impl<W: Write> Draw<W> {
    pub(crate) fn clear_screen<T: Terminal>(&mut self) {
        writeln!(self.stdout, "{}", T::clear_all()).unwrap();
    }

    pub(crate) fn default_bg() -> Black {
        Black
    }
    
    pub(crate) fn default_fg() -> LightWhite {
        LightWhite
    }

    pub(crate) fn set_colors<C1: Color, C2: Color>(&mut self, foreground: C1, background: C2) {
        writeln!(
            self.stdout,
            "{}{}",
            foreground.fg_code(),
            background.bg_code(),
        )
        .unwrap();
    }

    pub(crate) fn draw_box(&mut self, col1: usize, row1: usize, col2: usize, row2: usize) {
        use std::cmp::{max, min};
        for col in min(col1, col2)..=max(col1, col2) {
            for row in min(row1, row2)..=max(row1, row2) {
                self.draw_text::<Stdout>(col, row, "█");
            }
        }
    }

    pub fn draw_text<T: Terminal>(&mut self, col: usize, row: usize, text: &str) {
        let col = u16::try_from(col).expect("column should fit in a u16");
        let row = u16::try_from(row).expect("row should fit in a u16");

        writeln!(self.stdout, "{}{}", T::goto(col, row), text).unwrap();
    }

    pub fn set_up_terminal<T: Terminal>(&mut self) {
        writeln!(
            self.stdout,
            "{}{}{}{}{}",
            Self::default_fg().fg_code(),
            Self::default_bg().bg_code(),
            T::clear_all(),
            T::goto(1, 1),
            T::hide(),
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn restore_terminal<T: Terminal>(&mut self) {
        writeln!(
            self.stdout,
            "{}{}{}{}{}",
            Reset.fg_code(),
            Reset.bg_code(),
            T::clear_all(),
            T::goto(1, 1),
            T::show(),
        )
        .unwrap();
        self.stdout.flush().unwrap();
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
        self.set_colors(LightBlue, Self::default_bg());
        self.draw_centered_box(WIDTH, height + 2);
        self.set_colors(White, Self::default_bg());
        self.draw_centered_box(WIDTH - 2, height);

        self.set_colors(LightBlack, White);
        let (col, mut row, _, _) = Self::centered_box_corners(WIDTH - 2, height);

        for line in lines.split('\n') {
            self.draw_text::<Stdout>(col, row, line);
            row += 1;
        }
    }
}
