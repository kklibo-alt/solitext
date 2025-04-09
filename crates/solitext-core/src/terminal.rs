#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    LightRed,
    LightYellow,
    LightGreen,
    LightBlue,
    LightWhite,
    LightBlack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Esc,
    Char(char),
    Ctrl(char),
    Unknown,
}

pub trait Terminal {
    fn set_colors2(&mut self, foreground: Color, background: Color);
    fn default_bg() -> Color;
    fn default_fg() -> Color;
    fn draw_text(&mut self, col: usize, row: usize, text: &str);
    fn set_up_terminal(&mut self);
    fn restore_terminal(&mut self);
    fn clear_screen(&mut self);
    fn flush(&mut self);
    fn get_key(&mut self) -> Key;
}
