use crate::terminal::TermionTerminal;
use solitext_core::cards::Card;
use solitext_core::game_state::GameState;
use solitext_core::tui::Ui;

mod terminal;

fn main() {
    let mut game_state = GameState::init(Card::ordered_deck());
    let mut ui = Ui::new(TermionTerminal::new());
    ui.run(&mut game_state);
}
