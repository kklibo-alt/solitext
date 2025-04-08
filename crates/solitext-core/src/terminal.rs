pub enum Color {
    Red,
    Green,
    White,
}

pub trait Terminal {
    fn set_colors2(&mut self, foreground: Color, background: Color);
}

use crate::draw::Draw;
use std::io::Write;
use termion::color;

fn to_termion_color(color: Color) -> Box<dyn color::Color> {
    match color {
        Color::Red => Box::new(color::Red),
        Color::Green => Box::new(color::Green),
        Color::White => Box::new(color::White),
    }
}

impl Terminal for Draw {
    fn set_colors2(&mut self, foreground: Color, background: Color) {
        let foreground = to_termion_color(foreground);
        let background = to_termion_color(background);

        writeln!(
            self.stdout,
            "{}{}",
            color::Fg(foreground.as_ref()),
            color::Bg(background.as_ref()),
        )
        .unwrap();
    }
}
