pub mod cards;
pub mod draw;
pub mod game_logic;
pub mod game_state;
pub mod selection;
pub mod terminal;
pub mod terminal_mock;
pub mod tui;

#[cfg(test)]
mod tests {
    use crate::cards::Card;
    use crate::game_state::GameState;
    use crate::terminal_mock::{MockInput, MockStdout};
    use crate::tui::Ui;

    #[test]
    fn test_generic_architecture() {
        // Create a UI with mock implementations
        let ui = Ui::<MockStdout, MockInput>::new();
        
        // Ensure we can create a game state
        let _game_state = GameState::init(Card::ordered_deck());
        
        // Verify the test compiles, which means our generic architecture is working
        assert!(true);
    }
}
