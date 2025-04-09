use solitext_core::terminal::{Color, Key, Terminal};
use std::io::Write;
use std::io::{Stdout, stdin, stdout};
use termion::event;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor};
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

    pub fn from_termion_key(key: event::Key) -> Key {
        match key {
            event::Key::Up => Key::Up,
            event::Key::Down => Key::Down,
            event::Key::Left => Key::Left,
            event::Key::Right => Key::Right,
            event::Key::Home => Key::Home,
            event::Key::End => Key::End,
            event::Key::Esc => Key::Esc,
            event::Key::Char(c) => Key::Char(c),
            event::Key::Ctrl(c) => Key::Ctrl(c),
            _ => Key::Unknown,
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

    fn default_bg() -> Color {
        Color::Black
    }
    fn default_fg() -> Color {
        Color::LightWhite
    }

    fn draw_text(&mut self, col: usize, row: usize, text: &str) {
        let col = u16::try_from(col).expect("column should fit in a u16");
        let row = u16::try_from(row).expect("row should fit in a u16");

        writeln!(self.stdout, "{}{}", cursor::Goto(col, row), text).unwrap();
    }

    fn set_up_terminal(&mut self) {
        writeln!(
            self.stdout,
            "{}{}{}{}{}",
            color::Fg(Self::to_termion_color(Self::default_fg()).as_ref()),
            color::Bg(Self::to_termion_color(Self::default_bg()).as_ref()),
            clear::All,
            cursor::Goto(1, 1),
            cursor::Hide,
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn restore_terminal(&mut self) {
        writeln!(
            self.stdout,
            "{}{}{}{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            clear::All,
            cursor::Goto(1, 1),
            cursor::Show,
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn clear_screen(&mut self) {
        writeln!(self.stdout, "{}", clear::All,).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn get_key(&mut self) -> Key {
        Self::from_termion_key(stdin().keys().next().unwrap().unwrap())
    }
}
