//! Common drawing code.

use super::Draw;
use std::io::Write;
use crate::terminal::{TerminalColor, termion_impl::TermionColor};

impl Draw {
    pub(crate) fn clear_screen(&mut self) {
        self.terminal.clear_screen();
    }

    pub(crate) fn default_bg(&self) -> Box<dyn TerminalColor> {
        self.terminal.default_bg()
    }
    
    pub(crate) fn default_fg(&self) -> Box<dyn TerminalColor> {
        self.terminal.default_fg()
    }

    pub(crate) fn set_colors(
        &mut self,
        foreground: &Box<dyn TerminalColor>,
        background: &Box<dyn TerminalColor>,
    ) {
        self.terminal.set_colors(&**foreground, &**background);
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

        self.terminal.goto(col, row);
        write!(self.terminal, "{}", text).unwrap();
    }

    pub fn set_up_terminal(&mut self) {
        self.terminal.clear_screen();
        self.terminal.goto(1, 1);
        self.terminal.hide_cursor();
        
        let fg = self.default_fg();
        let bg = self.default_bg();
        self.terminal.set_colors(&*fg, &*bg);
        
        self.terminal.flush().unwrap();
    }

    pub fn restore_terminal(&mut self) {
        self.terminal.reset_colors();
        self.terminal.clear_screen();
        self.terminal.goto(1, 1);
        self.terminal.show_cursor();
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
        // Draw outer blue box
        let light_blue = TermionColor::light_blue();
        let black_bg = TermionColor::black();
        self.set_colors(&light_blue, &black_bg);
        self.draw_centered_box(WIDTH, height + 2);
        
        // Draw inner white box
        let white = TermionColor::white();
        let black_bg = TermionColor::black();
        self.set_colors(&white, &black_bg);
        self.draw_centered_box(WIDTH - 2, height);

        // Draw text with black on white
        let light_black = TermionColor::light_black();
        let white_bg = TermionColor::white();
        self.set_colors(&light_black, &white_bg);
        let (col, mut row, _, _) = Self::centered_box_corners(WIDTH - 2, height);

        for line in lines.split('\n') {
            self.draw_text(col, row, line);
            row += 1;
        }
    }
}
