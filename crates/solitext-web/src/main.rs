mod terminal_impl;

use solitext_core::cards::Card;
use solitext_core::game_state::GameState;
use solitext_core::tui::Ui;
use terminal_impl::WebInput;
use std::io::Stdout;

fn main() {
    // For now, we'll just set up a basic game structure 
    // In a real implementation, this would connect to the web UI
    let mut game_state = GameState::init(Card::ordered_deck());
    
    // Create a UI with our web terminal implementation
    let mut ui = Ui::<Stdout>::new();
    
    ui.run(&mut game_state);
}
