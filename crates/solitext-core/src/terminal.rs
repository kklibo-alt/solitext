pub enum Color {
    Red,
    Green,
    White,
}

pub trait Terminal {
    fn set_colors2(&mut self, foreground: Color, background: Color);
}
