# Refactoring terminal.rs for Cross-Platform Support

This guide outlines the steps to refactor the terminal.rs module to support both termion (for native/local builds) and ratzilla (for web builds) without using feature flags.

## Steps

1. Create a trait-based abstraction layer in solitext-core:
   - Convert the existing `terminal.rs` into a pure trait definition module
   - Remove all termion-specific implementation details
   - Define platform-agnostic traits for terminal functionality

2. Define the common interface in solitext-core/src/terminal.rs:
   ```rust
   // Platform-agnostic terminal key definitions
   pub enum Key {
       Char(char),
       Ctrl(char),
       Alt(char),
       Left,
       Right,
       Up,
       Down,
       Home,
       End,
       Backspace,
       Delete,
       Enter,
       Tab,
       Esc,
       // ... other keys as needed
   }

   // Color trait
   pub trait Color {
       fn as_fg_str(&self) -> String;
       fn as_bg_str(&self) -> String;
   }

   // Standard colors
   pub struct Black;
   pub struct Red;
   pub struct Green;
   pub struct Yellow;
   pub struct Blue;
   pub struct Magenta;
   pub struct Cyan;
   pub struct White;
   pub struct LightBlack;
   pub struct LightRed;
   pub struct LightGreen;
   pub struct LightYellow;
   pub struct LightBlue;
   pub struct LightMagenta;
   pub struct LightCyan;
   pub struct LightWhite;
   pub struct Reset;

   // Terminal interface
   pub trait Terminal {
       // Raw mode handling
       type RawTerminal;
       fn into_raw_mode(self) -> Self::RawTerminal;
       
       // Cursor operations
       fn goto(x: u16, y: u16) -> String;
       fn hide() -> String;
       fn show() -> String;
       
       // Screen operations
       fn clear_all() -> String;
   }

   // Input handling
   pub trait TerminalInput {
       type Keys;
       fn keys(self) -> Self::Keys;
       fn next_key(keys: &mut Self::Keys) -> Option<Result<Key, std::io::Error>>;
   }
   ```

3. Move the termion implementation to solitext-local:
   - Create `solitext-local/src/terminal_impl.rs` to implement the traits using termion
   - This implementation translates between termion's types and the core traits

   ```rust
   use solitext_core::terminal::{Key, Color, Terminal, TerminalInput};
   use std::io::{stdin, stdout, Stdout, Write};
   use termion;
   
   // Map termion Keys to our Key enum
   pub fn map_termion_key(key: termion::event::Key) -> Key {
       match key {
           termion::event::Key::Char(c) => Key::Char(c),
           termion::event::Key::Ctrl(c) => Key::Ctrl(c),
           // ... other mappings
       }
   }
   
   // Implement Color for each color type
   impl Color for solitext_core::terminal::Black {
       fn as_fg_str(&self) -> String {
           format!("{}", termion::color::Fg(termion::color::Black))
       }
       fn as_bg_str(&self) -> String {
           format!("{}", termion::color::Bg(termion::color::Black))
       }
   }
   
   // ... implement for other colors
   
   // Implement Terminal for Stdout
   impl Terminal for Stdout {
       type RawTerminal = termion::raw::RawTerminal<Stdout>;
       
       fn into_raw_mode(self) -> Self::RawTerminal {
           termion::raw::IntoRawMode::into_raw_mode(self).unwrap()
       }
       
       fn goto(x: u16, y: u16) -> String {
           format!("{}", termion::cursor::Goto(x, y))
       }
       
       fn hide() -> String {
           format!("{}", termion::cursor::Hide)
       }
       
       fn show() -> String {
           format!("{}", termion::cursor::Show)
       }
       
       fn clear_all() -> String {
           format!("{}", termion::clear::All)
       }
   }
   
   // Implement TerminalInput
   impl TerminalInput for std::io::Stdin {
       type Keys = termion::input::Keys<std::io::Stdin>;
       
       fn keys(self) -> Self::Keys {
           termion::input::TermRead::keys(self)
       }
       
       fn next_key(keys: &mut Self::Keys) -> Option<Result<Key, std::io::Error>> {
           keys.next().map(|res| res.map(map_termion_key))
       }
   }
   ```

4. Create the ratzilla implementation in solitext-web:
   - Create `solitext-web/src/terminal_impl.rs` to implement the traits using ratzilla
   - This implementation translates between ratzilla's types and the core traits

   ```rust
   use solitext_core::terminal::{Key, Color, Terminal, TerminalInput};
   use ratzilla;
   
   // Similar implementations but using ratzilla instead of termion
   // ...
   ```

5. Update the draw module in solitext-core to use the trait-based API:
   ```rust
   // In draw.rs
   use crate::terminal::{Terminal, Color};
   use std::io::{Stdout, stdout, Write};
   
   pub struct Draw<T: Terminal + Write> {
       stdout: T::RawTerminal,
       // ... other fields
   }
   
   impl<T: Terminal + Write> Draw<T> {
       pub fn new() -> Self {
           Self {
               stdout: stdout().into_raw_mode(),
               // ... other initializations
           }
       }
       
       // ... implement methods using the trait API
   }
   ```

6. Wire up the implementations in the executables:
   - In solitext-local/src/main.rs:
   ```rust
   mod terminal_impl;
   
   use solitext_core::draw::Draw;
   use solitext_core::tui::Ui;
   use std::io::stdout;
   
   fn main() {
       // Initialize with termion implementation
       let ui = Ui::new();
       ui.run();
   }
   ```

   - In solitext-web/src/main.rs:
   ```rust
   mod terminal_impl;
   
   use solitext_core::draw::Draw;
   use solitext_core::tui::Ui;
   use ratzilla;
   
   fn main() {
       // Initialize with ratzilla implementation
       let ui = Ui::new();
       ui.run();
   }
   ```

7. Update the main tui module to be generic:
   ```rust
   pub struct Ui<T: Terminal + TerminalInput + Write> {
       draw: Draw<T>,
       // ... other fields
   }

   impl<T: Terminal + TerminalInput + Write> Ui<T> {
       pub fn new() -> Self {
           Self {
               draw: Draw::new(),
               // ... other initializations
           }
       }
       
       // ... other methods using the traits
   }
   ```

8. Create stub/mock implementations for unit tests:
   - Add a terminal_mock.rs implementation for testing

9. Test the application on both platforms:
   - Verify solitext-local works correctly with the termion implementation
   - Verify solitext-web works correctly with the ratzilla implementation

10. Document the platform abstraction:
    - Add comments to the terminal.rs file explaining the design
    - Document how to add support for additional backends in the future 