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

pub trait Terminal {
    fn set_colors2(&mut self, foreground: Color, background: Color);
}
