mod terminal_impl;

use solitext_core::cards::Card;
use solitext_core::game_state::GameState;
use solitext_core::tui::Ui;
use std::io::Stdout;
use terminal_impl::Stdin;

fn main() {
    let mut game_state = GameState::init(Card::ordered_deck());
    
    // Create a UI with the stdout terminal implementation and stdin input
    let mut ui = Ui::<Stdout, Stdin>::new();
    
    ui.run(&mut game_state);
}
