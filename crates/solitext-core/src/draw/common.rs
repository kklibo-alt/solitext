//! Common drawing code.

use super::Draw;
use crate::terminal::{Terminal, Color};
use std::io::Write;

// Implementation for any terminal implementation that implements Terminal + Write
impl<T> Draw<T>
where 
    T: Terminal + Write,
    T::RawTerminal: Write,
{
    // Clear the screen
    pub(crate) fn clear_screen(&mut self) {
        writeln!(self.stdout, "{}", T::clear_all()).unwrap();
    }

    // Get the default background color
    pub(crate) fn default_bg<C: Color>(&self, color: C) -> C {
        color
    }
    
    // Get the default foreground color
    pub(crate) fn default_fg<C: Color>(&self, color: C) -> C {
        color
    }

    // Set colors
    pub(crate) fn set_colors<C1: Color, C2: Color>(&mut self, foreground: C1, background: C2) {
        writeln!(
            self.stdout,
            "{}{}",
            foreground.fg_code(),
            background.bg_code(),
        )
        .unwrap();
    }

    // Draw a box
    pub(crate) fn draw_box(&mut self, col1: usize, row1: usize, col2: usize, row2: usize) {
        use std::cmp::{max, min};
        for col in min(col1, col2)..=max(col1, col2) {
            for row in min(row1, row2)..=max(row1, row2) {
                self.draw_text(col, row, "█");
            }
        }
    }

    // Draw text at a position
    pub fn draw_text(&mut self, col: usize, row: usize, text: &str) {
        let col = u16::try_from(col).expect("column should fit in a u16");
        let row = u16::try_from(row).expect("row should fit in a u16");

        writeln!(self.stdout, "{}{}", T::goto(col, row), text).unwrap();
    }

    // Set up the terminal for drawing
    pub fn set_up_terminal<BG: Color, FG: Color>(&mut self, fg: FG, bg: BG) {
        writeln!(
            self.stdout,
            "{}{}{}{}{}",
            fg.fg_code(),
            bg.bg_code(),
            T::clear_all(),
            T::goto(1, 1),
            T::hide(),
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    // Restore the terminal to its normal state
    pub fn restore_terminal<R: Color>(&mut self, reset: R) {
        writeln!(
            self.stdout,
            "{}{}{}{}{}",
            reset.fg_code(),
            reset.bg_code(),
            T::clear_all(),
            T::goto(1, 1),
            T::show(),
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    // Calculate the corners of a centered box
    fn centered_box_corners(width: usize, height: usize) -> (usize, usize, usize, usize) {
        const CENTER: (usize, usize) = (26, 5);
        (
            CENTER.0 - width / 2,
            CENTER.1 - height / 2,
            CENTER.0 + width / 2,
            CENTER.1 + height / 2,
        )
    }

    // Draw a centered box
    fn draw_centered_box(&mut self, width: usize, height: usize) {
        let (col1, row1, col2, row2) = Self::centered_box_corners(width, height);
        self.draw_box(col1, row1, col2, row2);
    }

    // Draw a text box with the given lines
    pub fn draw_text_box<B1: Color, B2: Color, F1: Color, F2: Color>(
        &mut self, 
        lines: &str,
        border_fg: F1,
        border_bg: B1,
        content_fg: F2,
        content_bg: B2,
    ) {
        let height = lines.split('\n').count();

        const WIDTH: usize = 38;
        self.set_colors(border_fg, border_bg);
        self.draw_centered_box(WIDTH, height + 2);
        self.set_colors(content_fg, content_bg);
        self.draw_centered_box(WIDTH - 2, height);

        let (col, mut row, _, _) = Self::centered_box_corners(WIDTH - 2, height);

        for line in lines.split('\n') {
            self.draw_text(col, row, line);
            row += 1;
        }
    }
}
