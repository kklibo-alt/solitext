use crate::cards::Card;
use crate::draw::Draw;
use crate::game_logic;
use crate::game_state::{GameMode, GameState};
use crate::selection::Selection;
use crate::terminal::Key;

pub struct Ui {
    /// The deck used to seed the current game (if any)
    game_deck: Option<Vec<Card>>,
    ui_state: UiState,
    draw: Draw,
}

enum UiState {
    StartScreen,
    NewGame(GameMode),
    RestartGame,
    Game,
    Victory,
    Quit,
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui {
    pub fn new() -> Self {
        Self {
            game_deck: None,
            ui_state: UiState::StartScreen,
            draw: Draw::new(),
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
        // Hit if the deck has cards and the drawn deck is empty
        // game_state.auto_hit(); [disabled; should remove permanently?]
        // Fix column selections, if needed
        self.apply_column_selection_rules(game_state);
        // Update context help line
        self.set_context_help_message();

        // (Any other automatic state changes can go here too)

        if game_logic::victory(game_state) {
            self.draw.debug_message = "Victory".to_string();
            self.ui_state = UiState::Victory;
            return true;
        }

        self.draw.display_game_state(game_state);
        false
    }

    fn run_game(&mut self, game_state: &mut GameState) {
        if self.turn_actions(game_state) {
            return;
        }

        while let Some(key) = self.draw.terminal.read_single_key() {
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
                        return;
                    }
                }
                Key::CtrlC => {
                    self.ui_state = UiState::Quit;
                    return;
                }
                _ => {}
            }
            if self.turn_actions(game_state) {
                return;
            }
        }
    }

    fn run_start_screen(&mut self) {
        self.draw.display_start_screen();
        
        while let Some(key) = self.draw.terminal.read_single_key() {
            match key {
                Key::Char('1') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawOne);
                    break;
                }
                Key::Char('3') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawThree);
                    break;
                }
                Key::Esc | Key::CtrlC => {
                    self.ui_state = UiState::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    /// Returns: true IFF UiState has changed
    fn run_game_menu(&mut self, game_state: &mut GameState) -> bool {
        self.draw.display_game_menu(game_state);
        
        while let Some(key) = self.draw.terminal.read_single_key() {
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
                Key::Char('q') | Key::CtrlC => {
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

    fn run_help(&mut self, game_state: &mut GameState) {
        self.draw.display_help(game_state);
        
        while let Some(key) = self.draw.terminal.read_single_key() {
            match key {
                Key::Esc | Key::Char('h') | Key::Char('q') | Key::CtrlC => {
                    break;
                }
                _ => {}
            }
        }
        
        self.draw.display_game_state(game_state);
    }

    fn run_victory(&mut self, game_state: &mut GameState) {
        self.draw.display_victory(game_state);
        
        while let Some(key) = self.draw.terminal.read_single_key() {
            match key {
                Key::Char('1') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawOne);
                    break;
                }
                Key::Char('3') => {
                    self.ui_state = UiState::NewGame(GameMode::DrawThree);
                    break;
                }
                Key::Esc | Key::Char('q') | Key::CtrlC => {
                    self.ui_state = UiState::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    /// Run the main UI loop
    pub fn run(&mut self) {
        self.draw.set_up_terminal();

        let deck = Card::shuffled_deck();
        let mut game_state = GameState::init(deck);

        loop {
            match self.ui_state {
                UiState::StartScreen => self.run_start_screen(),
                UiState::NewGame(mode) => {
                    self.reset_for_new_game();
                    let deck = Card::shuffled_deck();
                    game_state = GameState::init(deck);
                    game_state.game_mode = mode;
                    self.ui_state = UiState::Game;
                }
                UiState::RestartGame => {
                    self.reset_for_new_game();
                    let deck = Card::shuffled_deck();
                    game_state = GameState::init(deck);
                    self.ui_state = UiState::Game;
                }
                UiState::Game => self.run_game(&mut game_state),
                UiState::Victory => self.run_victory(&mut game_state),
                UiState::Quit => break,
            }
        }

        self.draw.restore_terminal();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;

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
}
