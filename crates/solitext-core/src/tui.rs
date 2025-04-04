use crate::cards::Card;
use crate::draw::Draw;
use crate::game_logic;
use crate::game_state::{GameMode, GameState};
use crate::selection::Selection;
use crate::terminal::{Black, Blue, Color, Key, LightGreen, LightWhite, LightYellow, Reset, Terminal, TerminalInput};
use std::io::Write;
use std::marker::PhantomData;

// UI struct that is generic over both the terminal implementation and input source
pub struct Ui<T, I>
where 
    T: Terminal + Write + Default,
    T::RawTerminal: Write,
    I: TerminalInput + Default,
{
    /// The deck used to seed the current game (if any)
    game_deck: Option<Vec<Card>>,
    ui_state: UiState,
    draw: Draw<T>,
    _input_type: PhantomData<I>, // Used only for type parameters
}

enum UiState {
    StartScreen,
    NewGame(GameMode),
    RestartGame,
    Game,
    Victory,
    Quit,
}

impl<T, I> Default for Ui<T, I>
where 
    T: Terminal + Write + Default,
    T::RawTerminal: Write,
    I: TerminalInput + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I> Ui<T, I>
where 
    T: Terminal + Write + Default,
    T::RawTerminal: Write,
    I: TerminalInput + Default,
{
    pub fn new() -> Self {
        Self {
            game_deck: None,
            ui_state: UiState::StartScreen,
            draw: Draw::new(T::default()),
            _input_type: PhantomData,
        }
    }
    
    pub fn reset_for_new_game(&mut self) {
        self.draw.cursor = Selection::Deck;
        self.draw.selected = None;
        self.draw.debug_message.clear();
        self.draw.context_help_message.clear();
    }

    fn move_cards(from: Selection, to: Selection, game_state: &mut GameState) -> Result<(), ()> {
        if from.same_collection(to) {
            return Err(());
        }

        let cards = from
            .selected_collection(game_state)
            .take(from.card_count())?;

        to.selected_collection(game_state).receive(cards)?;
        Ok(())
    }

    fn cards_action(&mut self, game_state: &mut GameState) {
        if let (Some(from), to) = (self.draw.selected, self.draw.cursor) {
            self.draw.selected = None;

            if game_logic::valid_move(from, to, game_state).is_ok() {
                match Self::move_cards(from, to, game_state) {
                    Ok(_) => self.draw.debug_message = "move OK".to_string(),
                    Err(_) => self.draw.debug_message = "move attempt failed".to_string(),
                }
            } else {
                self.draw.debug_message = "invalid move".to_string();
            }
        } else if self.draw.cursor.card_count() > 0 {
            self.draw.selected = Some(self.draw.cursor);
        }
    }

    fn move_to_pile(from: Selection, game_state: &mut GameState) {
        for i in 0..4 {
            let to = Selection::Pile { index: i };
            if game_logic::valid_move(from, to, game_state).is_ok() {
                let _ = Self::move_cards(from, to, game_state);
                break;
            }
        }
    }

    fn enter_key_action(&mut self, game_state: &mut GameState) {
        if let Selection::Deck = self.draw.cursor {
            if let Some(Selection::Deck) = self.draw.selected {
                Self::move_to_pile(Selection::Deck, game_state);
            } else {
                game_state.deck_hit();
            }
        } else if let Selection::Column { index, .. } = self.draw.cursor {
            self.draw.cursor = Selection::Column {
                index,
                card_count: 1,
            };
            Self::move_to_pile(self.draw.cursor, game_state);
        }
        self.draw.selected = None;
    }

    fn debug_unchecked_cards_action(&mut self, game_state: &mut GameState) {
        if let Some(selected) = self.draw.selected {
            self.draw.selected = None;
            let _ = Self::move_cards(selected, self.draw.cursor, game_state);
        } else {
            self.draw.selected = Some(self.draw.cursor)
        }
    }

    fn debug_check_valid(&mut self, game_state: &mut GameState) {
        if let (Some(from), to) = (self.draw.selected, self.draw.cursor) {
            self.draw.debug_message = format!("{:?}", game_logic::valid_move(from, to, game_state));
        } else {
            self.draw.debug_message = "".to_string();
        }
    }

    fn apply_column_selection_rules(&mut self, game_state: &mut GameState) {
        self.draw
            .cursor
            .apply_column_selection_rules(game_state, self.draw.debug_mode);
        if let Some(mut selected) = self.draw.selected {
            selected.apply_column_selection_rules(game_state, self.draw.debug_mode);
        }
    }

    fn set_context_help_message(&mut self) {
        self.draw.context_help_message = match (self.draw.cursor, self.draw.selected) {
            (Selection::Column { .. }, _) | (Selection::Deck, Some(Selection::Deck)) => {
                "Enter: Try to Move to Stack"
            }
            (Selection::Deck, _) => "Enter: Hit",
            _ => "",
        }
        .to_string()
    }

    /// Actions run on each user turn
    /// Returns: true IFF UiState has changed
    fn turn_actions(&mut self, game_state: &mut GameState) -> bool {
        // Ensure a face-up card at the end of each column
        game_logic::face_up_on_columns(game_state);
        
        // Fix column selections, if needed
        self.apply_column_selection_rules(game_state);
        
        // Update context help line
        self.set_context_help_message();

        if game_logic::victory(game_state) {
            self.draw.debug_message = "Victory".to_string();
            self.ui_state = UiState::Victory;
            return true;
        }

        // Display game state with appropriate colors
        self.draw.display_game_state(
            game_state,
            LightWhite,  // foreground 
            Black,       // background
            Blue,        // accent background
            LightGreen,  // cursor accent 1
            LightYellow, // cursor accent 2
        );
        
        false
    }

    fn run_game(&mut self, game_state: &mut GameState) {
        if self.turn_actions(game_state) {
            return;
        }

        let input = I::default();
        let mut keys = input.keys();
        
        while let Some(result) = I::read_key(&mut keys) {
            let key = result.unwrap();
            match key {
                Key::Left => self.draw.cursor.move_left(),
                Key::Right => self.draw.cursor.move_right(),
                Key::Up => self.draw.cursor.select_up(),
                Key::Down => self.draw.cursor.select_down(),
                Key::Home => self.draw.cursor = Selection::Deck,
                Key::End => self.draw.cursor = Selection::Pile { index: 0 },
                Key::Char(' ') => self.cards_action(game_state),
                Key::Char('\n') => self.enter_key_action(game_state),
                Key::Char('c') if self.draw.debug_mode => {
                    self.debug_unchecked_cards_action(game_state)
                }
                Key::Char('x') => self.draw.selected = None,
                Key::Char('z') if self.draw.debug_mode => self.debug_check_valid(game_state),
                Key::Char('d') => self.draw.debug_mode = !self.draw.debug_mode,
                Key::Char('h') => self.run_help(game_state),
                Key::Esc => {
                    if self.run_game_menu(game_state) {
                        break;
                    }
                }
                Key::Ctrl('c') => {
                    self.ui_state = UiState::Quit;
                    break;
                }
                _ => {}
            }
            if self.turn_actions(game_state) {
                return;
            }
        }
    }

    fn run_start_screen(&mut self) {
        // Set up the terminal and display the start screen
        self.draw.set_up_terminal(LightWhite, Black);
        self.draw.display_start_screen(
            LightWhite, 
            Black, 
            Blue
        );

        // Get input from the user
        let input = I::default();
        let mut keys = input.keys();
        
        while let Some(result) = I::read_key(&mut keys) {
            let key = result.unwrap();
            match key {
                Key::Char('1') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawOne);
                    break;
                }
                Key::Char('3') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawThree);
                    break;
                }
                Key::Esc | Key::Ctrl('c') => {
                    self.ui_state = UiState::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    fn run_game_menu(&mut self, game_state: &mut GameState) -> bool {
        // Display the game menu
        self.draw.display_game_menu(
            game_state, 
            LightWhite, 
            Black, 
            Blue
        );
        
        // Get input from the user
        let input = I::default();
        let mut keys = input.keys();
        
        while let Some(result) = I::read_key(&mut keys) {
            let key = result.unwrap();
            match key {
                Key::Char('1') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawOne);
                    return true;
                }
                Key::Char('3') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawThree);
                    return true;
                }
                Key::Char('r') => {
                    self.ui_state = UiState::RestartGame;
                    return true;
                }
                Key::Char('q') | Key::Ctrl('c') => {
                    self.ui_state = UiState::Quit;
                    return true;
                }
                Key::Esc => {
                    return false;
                }
                _ => {}
            }
        }
        false
    }

    fn run_victory(&mut self, game_state: &mut GameState) {
        // Display the victory screen
        self.draw.display_victory(
            game_state, 
            LightWhite, 
            Black, 
            Blue
        );

        // Get input from the user
        let input = I::default();
        let mut keys = input.keys();
        
        while let Some(result) = I::read_key(&mut keys) {
            let key = result.unwrap();
            match key {
                Key::Char('y') => {
                    self.ui_state = UiState::NewGame(game_state.game_mode);
                    break;
                }
                Key::Char('n') | Key::Esc | Key::Ctrl('c') => {
                    self.ui_state = UiState::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn run_new_game(&mut self, game_state: &mut GameState, game_mode: GameMode) {
        let game_deck = Card::shuffled_deck();
        self.game_deck = Some(game_deck.clone());
        *game_state = GameState::init(game_deck);
        game_state.game_mode = game_mode;
        self.reset_for_new_game();
        self.ui_state = UiState::Game;
    }

    pub fn run_restart_game(&mut self, game_state: &mut GameState) {
        let game_mode = game_state.game_mode;
        *game_state = GameState::init(
            self.game_deck
                .clone()
                .expect("deck for current game should exist"),
        );
        game_state.game_mode = game_mode;
        self.reset_for_new_game();
        self.ui_state = UiState::Game;
    }

    pub fn run_help(&mut self, game_state: &mut GameState) {
        // Display the help screen
        self.draw.display_help(
            game_state, 
            LightWhite, 
            Black, 
            Blue
        );
        
        // Wait for any key press
        let input = I::default();
        let mut keys = input.keys();
        I::read_key(&mut keys); // Just wait for one key press
    }

    pub fn run(&mut self, game_state: &mut GameState) {
        // Set up the terminal
        self.draw.set_up_terminal(LightWhite, Black);

        loop {
            match self.ui_state {
                UiState::StartScreen => self.run_start_screen(),
                UiState::NewGame(game_mode) => self.run_new_game(game_state, game_mode),
                UiState::RestartGame => self.run_restart_game(game_state),
                UiState::Game => self.run_game(game_state),
                UiState::Victory => self.run_victory(game_state),
                UiState::Quit => break,
            }
        }

        // Restore the terminal
        self.draw.restore_terminal(Reset);
        
        // Final message
        self.draw.draw_text(1, 1, "please send bug reports via IRC or ham radio");
        self.draw.draw_text(1, 2, "");
    }

    // Test-only methods
    #[cfg(test)]
    pub fn get_ui_state_for_test(&self) -> String {
        match self.ui_state {
            UiState::StartScreen => "StartScreen".to_string(),
            UiState::NewGame(_) => "NewGame".to_string(),
            UiState::RestartGame => "RestartGame".to_string(),
            UiState::Game => "Game".to_string(),
            UiState::Victory => "Victory".to_string(),
            UiState::Quit => "Quit".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;
    use crate::terminal_mock::{MockStdout, MockInput};

    #[test]
    fn test_same_collection() {
        assert!(!Selection::Deck.same_collection(Selection::Column {
            index: 1,
            card_count: 1
        }));
        assert!(
            !Selection::Column {
                index: 1,
                card_count: 1
            }
            .same_collection(Selection::Pile { index: 1 })
        );
        assert!(!Selection::Pile { index: 1 }.same_collection(Selection::Deck));

        assert!(Selection::Deck.same_collection(Selection::Deck));

        assert!(
            Selection::Column {
                index: 1,
                card_count: 1
            }
            .same_collection(Selection::Column {
                index: 1,
                card_count: 1
            })
        );
        assert!(
            Selection::Column {
                index: 1,
                card_count: 2
            }
            .same_collection(Selection::Column {
                index: 1,
                card_count: 1
            })
        );
        assert!(
            !Selection::Column {
                index: 2,
                card_count: 1
            }
            .same_collection(Selection::Column {
                index: 1,
                card_count: 1
            })
        );

        assert!(Selection::Pile { index: 1 }.same_collection(Selection::Pile { index: 1 }));
        assert!(!Selection::Pile { index: 2 }.same_collection(Selection::Pile { index: 1 }));
    }

    #[test]
    fn test_selected_collection() {
        let mut a = GameState::init(Card::ordered_deck());
        let _b = Selection::Deck.selected_collection(&mut a);
    }
    
    #[test]
    fn test_ui_with_mock() {
        let mut ui = Ui::<MockStdout, MockInput>::new();
        let mut game_state = GameState::new_with_seed(0, GameMode::DrawOne);
        
        // Just make sure we can create and run the UI with mocks
        ui.reset_for_new_game();
        ui.turn_actions(&mut game_state);
        
        assert_eq!(ui.draw.cursor, Selection::Deck);
    }
}
