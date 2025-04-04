# Solitext

A terminal-based Solitaire game with multiple platform support.

## Architecture

Solitext uses a modular architecture based on Rust's trait system to support multiple platforms:

- **Core Module**: Contains the game logic, state management, and drawing interfaces
- **Local Implementation**: Uses termion for terminal I/O on native platforms
- **Web Implementation**: Uses ratzilla for web-based terminal emulation

The application uses generic traits to abstract terminal operations:

- `Terminal`: For basic terminal operations (cursor movement, colors, etc.)
- `TerminalInput`: For handling user input
- `Color`: For handling terminal colors

This architecture allows us to have a single codebase that works across different platforms without feature flags.

## Running the Game

### Local Terminal

```
cargo run --bin local
```

### Web Browser

```
cd crates/solitext-web
cargo run
# Then open your browser to the URL displayed
```

## Development

To add support for a new terminal backend:

1. Create a new implementation of the `Terminal` trait
2. Implement the `Color` trait for all color types
3. Implement the `TerminalInput` trait for your input handling
4. Wire up your implementation in your crate's main.rs

See the implementations in `solitext-local` and `solitext-web` for examples.

## License

MIT 