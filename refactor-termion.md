# Refactoring Plan: Consolidate termion References to a Single Module

## Steps

1. Create a new module named `terminal.rs` in `crates/solitext-core/src/` to encapsulate all termion functionality.

2. In the new `terminal.rs` module, create appropriate abstractions for:
   - Color handling
   - Raw terminal mode
   - Key event handling
   - Cursor positioning
   - Terminal clearing
   - Input reading

3. Update `lib.rs` to expose the new terminal module:
   ```rust
   pub mod cards;
   pub mod draw;
   pub mod game_logic;
   pub mod game_state;
   pub mod selection;
   pub mod terminal;
   pub mod tui;
   ```

4. Refactor `tui.rs` to use the terminal module instead of termion directly:
   - Replace `use termion::event::Key;` with `use crate::terminal::Key;`
   - Replace `use termion::input::TermRead;` with `use crate::terminal::TermReader;`

5. Refactor `draw.rs` to use the terminal module:
   - Replace `use termion::raw::{IntoRawMode, RawTerminal};` with `use crate::terminal::{IntoRawMode, RawTerminal};`
   - Update the Draw struct to use the terminal-abstracted types

6. Refactor draw submodules to use the terminal module:
   - Update `draw/common.rs` to use `crate::terminal` instead of `termion`
   - Update `draw/card.rs` to use terminal color abstractions
   - Update `draw/card_column.rs` to use terminal color abstractions
   - Update `draw/deck.rs` to use terminal color abstractions
   - Update `draw/foundation.rs` to use terminal color abstractions
   - Update `draw/game_state.rs` to use terminal color abstractions
   - Update `draw/info.rs` to use terminal color abstractions

7. Implement the terminal module with abstractions that maintain the existing functionality:
   ```rust
   // terminal.rs
   // Re-export necessary termion types with our own abstractions
   
   // For keys and input handling
   pub use termion::event::Key;
   
   // For terminal mode
   pub use termion::raw::{IntoRawMode, RawTerminal};
   
   // For cursor and terminal control
   pub mod cursor {
       pub use termion::cursor::{Goto, Hide, Show};
   }
   
   pub mod clear {
       pub use termion::clear::All;
   }
   
   // For input handling
   pub trait TermReader {
       fn keys(&self) -> termion::input::Keys<std::io::Stdin>;
   }
   
   impl TermReader for std::io::Stdin {
       fn keys(&self) -> termion::input::Keys<std::io::Stdin> {
           termion::input::TermRead::keys(self)
       }
   }
   
   // For colors
   pub mod color {
       pub use termion::color::{self, Bg, Fg, Color, Reset, Black, White, LightWhite, LightBlack, LightBlue, Red, LightRed, Green, LightGreen};
   }
   ```

8. Test each component after refactoring to ensure functionality is preserved.

9. Update any tests that rely on termion directly to use the terminal module.

10. Once all references to termion are removed from other modules and all tests pass, update the `Cargo.toml` to ensure termion is only a dependency of the solitext-core crate and not exposed in the public API.

## Benefits

- Isolates terminal-specific code to a single module
- Makes it easier to swap out termion for a different terminal library in the future
- Creates a cleaner abstraction boundary between UI rendering and core game logic
- Simplifies cross-platform support by containing platform-specific terminal code 