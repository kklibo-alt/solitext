use solitext_core::terminal::{Color, Key, Terminal};

pub struct RatzillaTerminal {}

impl Terminal for RatzillaTerminal {
    fn set_colors2(&mut self, _foreground: Color, _background: Color) {
        unimplemented!()
    }

    fn default_bg() -> Color {
        unimplemented!()
    }

    fn default_fg() -> Color {
        unimplemented!()
    }

    fn draw_text(&mut self, _col: usize, _row: usize, _text: &str) {
        unimplemented!()
    }

    fn set_up_terminal(&mut self) {
        unimplemented!()
    }

    fn restore_terminal(&mut self) {
        unimplemented!()
    }

    fn clear_screen(&mut self) {
        unimplemented!()
    }

    fn flush(&mut self) {
        unimplemented!()
    }

    fn get_key(&mut self) -> Key {
        unimplemented!()
    }
}
