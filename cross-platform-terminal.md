# Refactoring terminal.rs for Cross-Platform Support

This guide outlines the steps to refactor the terminal.rs module to support both termion (for native/local builds) and ratzilla (for web builds).

## Steps

1. Create a trait-based abstraction layer in solitext-core:
   - Rename the existing `terminal.rs` to `terminal/mod.rs`
   - Define platform-agnostic traits for terminal functionality

2. Create feature flags in solitext-core's Cargo.toml:
   ```toml
   [features]
   default = []
   ```

3. Extract the common interface in terminal/mod.rs:
   ```rust
   // Platform-agnostic traits
   pub trait TerminalKey {
       // Key event types and constants
   }

   pub trait TerminalColor {
       // Color handling
   }

   pub trait TerminalCursor {
       // Cursor positioning
   }

   pub trait TerminalInput {
       // Input handling
   }

   pub trait TerminalRaw {
       // Raw terminal mode
   }

   // Re-export concrete implementations based on current platform
   // (These will come from submodules)
   ```

4. Create terminal/termion.rs to implement the traits using termion:
   ```rust
   use super::*;
   use termion;
   
   // Implement all terminal traits using termion
   ```

5. Create terminal/ratzilla.rs to implement the traits using ratzilla:
   ```rust
   use super::*;
   use ratzilla;
   
   // Implement all terminal traits using ratzilla
   ```

6. Update solitext-core/Cargo.toml with conditional dependencies:
   ```toml
   [dependencies]
   termion = { workspace = true, optional = true }
   ratzilla = { version = "0.1.0", optional = true }

   [features]
   default = ["termion"]
   termion-backend = ["termion"]
   ratzilla-backend = ["ratzilla"]
   ```

7. In terminal/mod.rs, use conditional compilation to re-export the appropriate implementation:
   ```rust
   #[cfg(feature = "termion-backend")]
   mod termion_impl;
   #[cfg(feature = "termion-backend")]
   pub use termion_impl::*;

   #[cfg(feature = "ratzilla-backend")]
   mod ratzilla_impl;
   #[cfg(feature = "ratzilla-backend")]
   pub use ratzilla_impl::*;
   ```

8. Update solitext-local/Cargo.toml to use the termion backend:
   ```toml
   [dependencies]
   solitext-core = { path = "../solitext-core", features = ["termion-backend"] }
   ```

9. Update solitext-web/Cargo.toml to use the ratzilla backend:
   ```toml
   [dependencies]
   solitext-core = { path = "../solitext-core", features = ["ratzilla-backend"] }
   ratzilla = "0.1.0"
   ```

10. Update the main workspace Cargo.toml to make termion optional:
    ```toml
    [workspace.dependencies]
    termion = { version = "4.0.5", optional = true }
    ```

11. Refactor the Draw implementation to use the trait-based API:
    - Update all modules that interact with terminal.rs to use the trait-based API
    - Fix any compiler errors that arise from the changes

12. Create stub/mock implementations for unit tests:
    - Add a terminal/mock.rs implementation for testing
    - Update tests to use the mock implementation

13. Implement platform-specific input handling:
    - For termion: use stdin().keys()
    - For ratzilla: use event listeners and state management

14. Test the application on both platforms:
    - Verify solitext-local works correctly with the termion backend
    - Verify solitext-web works correctly with the ratzilla backend

15. Document the platform abstraction:
    - Add README.md to the terminal module explaining the design
    - Document how to add support for additional backends in the future 