use solitext::cards::Card;
use solitext::game_state::GameState;
use solitext::tui::Ui;

fn main() {
    // Temp: just a copy of local.rs
    let mut game_state = GameState::init(Card::ordered_deck());
    let mut ui = Ui::new();
    ui.run(&mut game_state);
}
