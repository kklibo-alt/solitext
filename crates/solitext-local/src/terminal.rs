use solitext_core::terminal::{Color, Terminal};
use std::io::Write;
use std::io::{Stdout, stdout};
use termion::color;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct TermionTerminal {
    pub stdout: RawTerminal<Stdout>,
}
impl Default for TermionTerminal {
    fn default() -> Self {
        Self::new()
    }
}
impl TermionTerminal {
    pub fn new() -> Self {
        Self {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn to_termion_color(color: Color) -> Box<dyn color::Color> {
        match color {
            Color::Red => Box::new(color::Red),
            Color::Green => Box::new(color::Green),
            Color::Blue => Box::new(color::Blue),
            Color::White => Box::new(color::White),
            Color::Black => Box::new(color::Black),
            Color::LightRed => Box::new(color::LightRed),
            Color::LightYellow => Box::new(color::LightYellow),
            Color::LightGreen => Box::new(color::LightGreen),
            Color::LightBlue => Box::new(color::LightBlue),
            Color::LightWhite => Box::new(color::LightWhite),
            Color::LightBlack => Box::new(color::LightBlack),
        }
    }
}

impl Terminal for TermionTerminal {
    fn set_colors2(&mut self, foreground: Color, background: Color) {
        let foreground = Self::to_termion_color(foreground);
        let background = Self::to_termion_color(background);

        writeln!(
            self.stdout,
            "{}{}",
            color::Fg(foreground.as_ref()),
            color::Bg(background.as_ref()),
        )
        .unwrap();
    }
}
