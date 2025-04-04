use crate::terminal::{KeyEvent, Terminal, TerminalColor, TerminalFactory, TerminalKeys};
use std::io::{self, stdin, stdout, Stdin, Stdout, Write};
use termion::color;
use termion::cursor;
use termion::event::Key as TermionKey;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// A termion-based implementation of the Terminal interface
pub struct TermionTerminal {
    stdout: RawTerminal<Stdout>,
}

impl Write for TermionTerminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Terminal for TermionTerminal {
    fn set_fg(&mut self, color: TerminalColor) -> io::Result<()> {
        match color {
            TerminalColor::Reset => write!(self.stdout, "{}", color::Fg(color::Reset)),
            TerminalColor::Black => write!(self.stdout, "{}", color::Fg(color::Black)),
            TerminalColor::Red => write!(self.stdout, "{}", color::Fg(color::Red)),
            TerminalColor::Green => write!(self.stdout, "{}", color::Fg(color::Green)),
            TerminalColor::Yellow => write!(self.stdout, "{}", color::Fg(color::Yellow)),
            TerminalColor::Blue => write!(self.stdout, "{}", color::Fg(color::Blue)),
            TerminalColor::Magenta => write!(self.stdout, "{}", color::Fg(color::Magenta)),
            TerminalColor::Cyan => write!(self.stdout, "{}", color::Fg(color::Cyan)),
            TerminalColor::White => write!(self.stdout, "{}", color::Fg(color::White)),
            TerminalColor::LightBlack => write!(self.stdout, "{}", color::Fg(color::LightBlack)),
            TerminalColor::LightRed => write!(self.stdout, "{}", color::Fg(color::LightRed)),
            TerminalColor::LightGreen => write!(self.stdout, "{}", color::Fg(color::LightGreen)),
            TerminalColor::LightYellow => write!(self.stdout, "{}", color::Fg(color::LightYellow)),
            TerminalColor::LightBlue => write!(self.stdout, "{}", color::Fg(color::LightBlue)),
            TerminalColor::LightMagenta => write!(self.stdout, "{}", color::Fg(color::LightMagenta)),
            TerminalColor::LightCyan => write!(self.stdout, "{}", color::Fg(color::LightCyan)),
            TerminalColor::LightWhite => write!(self.stdout, "{}", color::Fg(color::LightWhite)),
        }
    }

    fn set_bg(&mut self, color: TerminalColor) -> io::Result<()> {
        match color {
            TerminalColor::Reset => write!(self.stdout, "{}", color::Bg(color::Reset)),
            TerminalColor::Black => write!(self.stdout, "{}", color::Bg(color::Black)),
            TerminalColor::Red => write!(self.stdout, "{}", color::Bg(color::Red)),
            TerminalColor::Green => write!(self.stdout, "{}", color::Bg(color::Green)),
            TerminalColor::Yellow => write!(self.stdout, "{}", color::Bg(color::Yellow)),
            TerminalColor::Blue => write!(self.stdout, "{}", color::Bg(color::Blue)),
            TerminalColor::Magenta => write!(self.stdout, "{}", color::Bg(color::Magenta)),
            TerminalColor::Cyan => write!(self.stdout, "{}", color::Bg(color::Cyan)),
            TerminalColor::White => write!(self.stdout, "{}", color::Bg(color::White)),
            TerminalColor::LightBlack => write!(self.stdout, "{}", color::Bg(color::LightBlack)),
            TerminalColor::LightRed => write!(self.stdout, "{}", color::Bg(color::LightRed)),
            TerminalColor::LightGreen => write!(self.stdout, "{}", color::Bg(color::LightGreen)),
            TerminalColor::LightYellow => write!(self.stdout, "{}", color::Bg(color::LightYellow)),
            TerminalColor::LightBlue => write!(self.stdout, "{}", color::Bg(color::LightBlue)),
            TerminalColor::LightMagenta => write!(self.stdout, "{}", color::Bg(color::LightMagenta)),
            TerminalColor::LightCyan => write!(self.stdout, "{}", color::Bg(color::LightCyan)),
            TerminalColor::LightWhite => write!(self.stdout, "{}", color::Bg(color::LightWhite)),
        }
    }

    fn reset_fg(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", color::Fg(color::Reset))
    }

    fn reset_bg(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", color::Bg(color::Reset))
    }

    fn clear_screen(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::clear::All)
    }

    fn goto(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.stdout, "{}", cursor::Goto(x, y))
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", cursor::Hide)
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", cursor::Show)
    }

    fn enter_raw_mode(&mut self) -> io::Result<()> {
        // Terminal is already in raw mode when created
        Ok(())
    }

    fn exit_raw_mode(&mut self) -> io::Result<()> {
        // Raw mode will be exited when the terminal is dropped
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

/// A termion-based implementation of the TerminalKeys interface
pub struct TermionKeys;

impl TerminalKeys for TermionKeys {
    fn next_key(&mut self) -> io::Result<Option<KeyEvent>> {
        // Create a fresh stdin for each call
        match stdin().keys().next() {
            Some(Ok(key)) => Ok(Some(convert_termion_key(key))),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

/// Convert termion's Key to our KeyEvent
fn convert_termion_key(key: TermionKey) -> KeyEvent {
    match key {
        TermionKey::Char(c) => KeyEvent::Char(c),
        TermionKey::Ctrl(c) => KeyEvent::Ctrl(c),
        TermionKey::Alt(c) => KeyEvent::Alt(c),
        TermionKey::Left => KeyEvent::Left,
        TermionKey::Right => KeyEvent::Right,
        TermionKey::Up => KeyEvent::Up,
        TermionKey::Down => KeyEvent::Down,
        TermionKey::Home => KeyEvent::Home,
        TermionKey::End => KeyEvent::End,
        TermionKey::PageUp => KeyEvent::PageUp,
        TermionKey::PageDown => KeyEvent::PageDown,
        TermionKey::Backspace => KeyEvent::Backspace,
        TermionKey::Delete => KeyEvent::Delete,
        TermionKey::Insert => KeyEvent::Insert,
        TermionKey::Esc => KeyEvent::Esc,
        TermionKey::F(n) => KeyEvent::F(n),
        TermionKey::Null => KeyEvent::Null,
        // Map any other keys to Null
        _ => KeyEvent::Null,
    }
}

/// Factory to create termion-based terminal and keys
pub struct TermionFactory;

impl TerminalFactory for TermionFactory {
    type Terminal = TermionTerminal;
    type Keys = TermionKeys;

    fn create_terminal(&self) -> io::Result<Self::Terminal> {
        let stdout = stdout().into_raw_mode()?;
        Ok(TermionTerminal { stdout })
    }

    fn create_keys(&self) -> Self::Keys {
        TermionKeys
    }
} 