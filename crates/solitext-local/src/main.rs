mod terminal_impl;

use solitext_core::cards::Card;
use solitext_core::game_state::GameState;
use solitext_core::tui::Ui;
use solitext_core::terminal::adapters::TerminalWrapper;
use terminal_impl::{LocalTerminal, Stdin};

fn main() {
    let mut game_state = GameState::init(Card::ordered_deck());
    
    // Create a UI with the local terminal implementation and stdin input
    let mut ui = Ui::<TerminalWrapper<LocalTerminal>, Stdin>::new();
    
    ui.run(&mut game_state);
}
