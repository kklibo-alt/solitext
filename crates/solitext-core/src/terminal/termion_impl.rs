//! Termion implementation of the Terminal interface

use std::io::{stdin, stdout, Stdout, Write};
use std::any::Any;
use termion::event;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor};

use super::{Key, Terminal, TerminalColor};

/// Wrapper around termion's Color trait to make it compatible
/// with our TerminalColor trait
pub struct TermionColor {
    /// The concrete color type, wrapped in an enum to handle all termion colors
    color_variant: TermionColorVariant,
    /// Whether this color is red (for card coloring)
    is_red_color: bool,
}

/// Enum to store the different concrete termion color types
enum TermionColorVariant {
    Red,
    Black,
    White,
    LightWhite,
    LightGreen,
    LightBlack,
    LightRed,
    LightBlue,
    Reset,
}

impl TermionColor {
    // Helper to write the foreground color to the given writer
    fn write_fg<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        match self.color_variant {
            TermionColorVariant::Red => write!(w, "{}", color::Fg(color::Red)),
            TermionColorVariant::Black => write!(w, "{}", color::Fg(color::Black)),
            TermionColorVariant::White => write!(w, "{}", color::Fg(color::White)),
            TermionColorVariant::LightWhite => write!(w, "{}", color::Fg(color::LightWhite)),
            TermionColorVariant::LightGreen => write!(w, "{}", color::Fg(color::LightGreen)),
            TermionColorVariant::LightBlack => write!(w, "{}", color::Fg(color::LightBlack)),
            TermionColorVariant::LightRed => write!(w, "{}", color::Fg(color::LightRed)),
            TermionColorVariant::LightBlue => write!(w, "{}", color::Fg(color::LightBlue)),
            TermionColorVariant::Reset => write!(w, "{}", color::Fg(color::Reset)),
        }
    }

    // Helper to write the background color to the given writer
    fn write_bg<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        match self.color_variant {
            TermionColorVariant::Red => write!(w, "{}", color::Bg(color::Red)),
            TermionColorVariant::Black => write!(w, "{}", color::Bg(color::Black)),
            TermionColorVariant::White => write!(w, "{}", color::Bg(color::White)),
            TermionColorVariant::LightWhite => write!(w, "{}", color::Bg(color::LightWhite)),
            TermionColorVariant::LightGreen => write!(w, "{}", color::Bg(color::LightGreen)),
            TermionColorVariant::LightBlack => write!(w, "{}", color::Bg(color::LightBlack)),
            TermionColorVariant::LightRed => write!(w, "{}", color::Bg(color::LightRed)),
            TermionColorVariant::LightBlue => write!(w, "{}", color::Bg(color::LightBlue)),
            TermionColorVariant::Reset => write!(w, "{}", color::Bg(color::Reset)),
        }
    }
}

impl TerminalColor for TermionColor {
    fn red() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::Red,
            is_red_color: true,
        })
    }

    fn black() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::Black,
            is_red_color: false,
        })
    }

    fn white() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::White,
            is_red_color: false,
        })
    }

    fn light_white() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::LightWhite,
            is_red_color: false,
        })
    }

    fn light_green() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::LightGreen,
            is_red_color: false,
        })
    }

    fn light_black() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::LightBlack,
            is_red_color: false,
        })
    }

    fn light_red() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::LightRed,
            is_red_color: true,
        })
    }

    fn light_blue() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::LightBlue,
            is_red_color: false,
        })
    }

    fn reset() -> Box<dyn TerminalColor> {
        Box::new(Self {
            color_variant: TermionColorVariant::Reset,
            is_red_color: false,
        })
    }

    fn is_red(&self) -> bool {
        self.is_red_color
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A termion implementation of the Terminal trait
pub struct TermionTerminal {
    stdout: RawTerminal<Stdout>,
}

impl Write for TermionTerminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

impl Terminal for TermionTerminal {
    fn new() -> Self {
        Self {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    fn clear_screen(&mut self) {
        writeln!(self.stdout, "{}", clear::All).unwrap();
    }

    fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
    }

    fn show_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap();
    }

    fn goto(&mut self, col: u16, row: u16) {
        write!(self.stdout, "{}", cursor::Goto(col, row)).unwrap();
    }

    fn set_colors(&mut self, fg: &dyn TerminalColor, bg: &dyn TerminalColor) {
        // We need to get the concrete termion colors from the trait objects
        let fg = fg.as_any().downcast_ref::<TermionColor>().unwrap();
        let bg = bg.as_any().downcast_ref::<TermionColor>().unwrap();
        
        fg.write_fg(&mut self.stdout).unwrap();
        bg.write_bg(&mut self.stdout).unwrap();
    }

    fn reset_colors(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
        .unwrap();
    }

    fn default_bg(&self) -> Box<dyn TerminalColor> {
        TermionColor::black()
    }

    fn default_fg(&self) -> Box<dyn TerminalColor> {
        TermionColor::light_white()
    }

    fn read_key(&self) -> Option<Key> {
        stdin().keys().next().and_then(|res| res.ok().map(convert_key))
    }

    fn keys(&self) -> Box<dyn Iterator<Item = Result<Key, std::io::Error>> + '_> {
        Box::new(
            stdin()
                .keys()
                .map(|r| r.map(convert_key)),
        )
    }
}

/// Convert a termion Key to our Key type
fn convert_key(key: event::Key) -> Key {
    match key {
        event::Key::Char(c) => {
            if c == 'c' && event::Key::Ctrl('c') == key {
                Key::CtrlC
            } else {
                Key::Char(c)
            }
        }
        event::Key::Left => Key::Left,
        event::Key::Right => Key::Right,
        event::Key::Up => Key::Up,
        event::Key::Down => Key::Down,
        event::Key::Home => Key::Home,
        event::Key::End => Key::End,
        event::Key::Esc => Key::Esc,
        event::Key::Ctrl('c') => Key::CtrlC,
        _ => Key::Char('\0'), // default for unsupported keys
    }
} 