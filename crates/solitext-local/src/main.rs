use solitext_core::cards::Card;
use solitext_core::game_state::GameState;
use solitext_core::tui::Ui;

mod terminal_termion;
use terminal_termion::TermionFactory;

fn main() {
    let mut game_state = GameState::init(Card::ordered_deck());
    let mut ui = Ui::new(TermionFactory);
    ui.run(&mut game_state);
}
