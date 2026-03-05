# Solitext Code Review

## High Priority

### `self.selected` mutation is lost (operates on copy)

In `app.rs:137-139`, `Selection` is `Copy`, so `if let Some(mut selected) = self.selected`
copies the value into a local. `apply_column_selection_rules` mutates the *copy*, which is
then dropped. `self.selected` is never updated. The fix would be
`if let Some(ref mut selected) = self.selected`. This bug also exists in the original
pre-port code.

### `test-case` is a regular dependency

In `Cargo.toml`, `test-case = "3.3.1"` is under `[dependencies]` rather than
`[dev-dependencies]`, so it's compiled into release builds. Furthermore, there are no
`#[test_case]` annotations anywhere in the codebase -- this dependency may be entirely
unused.

## Medium Priority

### `valid_move` takes `&mut GameState` but only reads

All validation functions in `game_logic.rs` take `&mut GameState` because
`selected_collection` returns `&mut dyn CardCollection`, even though validation only calls
`peek()`/`peek_n()`. This is a design issue from the original code that prevents calling
validation with shared references.

### `Result<(), ()>` everywhere

Errors carry no context. An error enum (e.g. `EmptySource`, `InvalidCount`, `WrongSuit`)
would make debugging easier, since currently the user just sees "move attempt failed".

### Duplicated key handling

The Game-screen key mappings in `tui.rs` and `web.rs` are nearly identical but manually
duplicated. Adding a keybinding to one is easy to forget in the other.

### No guard against both features enabled

If both `native` and `web` are enabled, you get a confusing "duplicate main" error. A
`compile_error!` for this case would be clearer.

## Low Priority

- **`#[allow(dead_code)]` should be `#[cfg(test)]`** for `Card::new`,
  `GameState::victory()`, `GameState::almost_victory()`. `GameState::auto_hit` appears
  completely unused.
- **Clone-heavy `deck_hit`** -- clones then clears when a `mem::swap` + reverse would
  suffice.
- **Tests for `Selection` are in `tui.rs`** instead of `selection.rs`.
- **Hardcoded layout constants** scattered across draw sub-modules don't adapt to terminal
  size.
- **Victory animation is native-only** -- web jumps straight to the final frame.
- **Potential arithmetic underflow** in `centered_box_corners` if given very large
  dimensions.
- **All `.unwrap()` on `terminal.draw()`** -- a terminal I/O error panics instead of
  gracefully exiting.
