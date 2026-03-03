use crate::cards::Card;
use crate::game_logic;
use crate::game_state::{GameMode, GameState};
use crate::selection::Selection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Start,
    Game,
    GameMenu,
    Help,
    Victory,
    Quit,
}

pub struct App {
    pub game_deck: Option<Vec<Card>>,
    pub screen: Screen,
    pub game_state: GameState,
    pub cursor: Selection,
    pub selected: Option<Selection>,
    pub context_help_message: String,
    pub debug_message: String,
    pub debug_mode: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            game_deck: None,
            screen: Screen::Start,
            game_state: GameState::init(Card::ordered_deck()),
            cursor: Selection::Deck,
            selected: None,
            context_help_message: String::new(),
            debug_message: String::new(),
            debug_mode: false,
        }
    }

    pub fn reset_for_new_game(&mut self) {
        self.cursor = Selection::Deck;
        self.selected = None;
        self.debug_message.clear();
        self.context_help_message.clear();
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

    pub fn cards_action(&mut self) {
        if let (Some(from), to) = (self.selected, self.cursor) {
            self.selected = None;
            if game_logic::valid_move(from, to, &mut self.game_state).is_ok() {
                match Self::move_cards(from, to, &mut self.game_state) {
                    Ok(_) => self.debug_message = "move OK".to_string(),
                    Err(_) => self.debug_message = "move attempt failed".to_string(),
                }
            } else {
                self.debug_message = "invalid move".to_string();
            }
        } else if self.cursor.card_count() > 0 {
            self.selected = Some(self.cursor);
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

    pub fn enter_key_action(&mut self) {
        if let Selection::Deck = self.cursor {
            if let Some(Selection::Deck) = self.selected {
                Self::move_to_pile(Selection::Deck, &mut self.game_state);
            } else {
                self.game_state.deck_hit();
            }
        } else if let Selection::Column { index, .. } = self.cursor {
            self.cursor = Selection::Column {
                index,
                card_count: 1,
            };
            Self::move_to_pile(self.cursor, &mut self.game_state);
        }
        self.selected = None;
    }

    pub fn debug_unchecked_cards_action(&mut self) {
        if let Some(selected) = self.selected {
            self.selected = None;
            let _ = Self::move_cards(selected, self.cursor, &mut self.game_state);
        } else {
            self.selected = Some(self.cursor)
        }
    }

    pub fn debug_check_valid(&mut self) {
        if let (Some(from), to) = (self.selected, self.cursor) {
            self.debug_message =
                format!("{:?}", game_logic::valid_move(from, to, &mut self.game_state));
        } else {
            self.debug_message = String::new();
        }
    }

    fn set_context_help_message(&mut self) {
        self.context_help_message = match (self.cursor, self.selected) {
            (Selection::Column { .. }, _) | (Selection::Deck, Some(Selection::Deck)) => {
                "Enter: Try to Move to Stack"
            }
            (Selection::Deck, _) => "Enter: Hit",
            _ => "",
        }
        .to_string();
    }

    /// Run game-state housekeeping after each action.
    /// Returns true if the screen changed (e.g. victory detected).
    pub fn update(&mut self) {
        game_logic::face_up_on_columns(&mut self.game_state);
        self.cursor
            .apply_column_selection_rules(&self.game_state, self.debug_mode);
        if let Some(mut selected) = self.selected {
            selected.apply_column_selection_rules(&self.game_state, self.debug_mode);
        }
        self.set_context_help_message();

        if self.screen == Screen::Game && game_logic::victory(&self.game_state) {
            self.debug_message = "Victory".to_string();
            self.screen = Screen::Victory;
        }
    }

    pub fn new_game(&mut self, mode: GameMode) {
        let game_deck = Card::shuffled_deck();
        self.game_deck = Some(game_deck.clone());
        self.game_state = GameState::init(game_deck);
        self.game_state.game_mode = mode;
        self.reset_for_new_game();
        self.screen = Screen::Game;
        self.update();
    }

    pub fn restart_game(&mut self) {
        let game_mode = self.game_state.game_mode;
        self.game_state = GameState::init(
            self.game_deck
                .clone()
                .expect("deck for current game should exist"),
        );
        self.game_state.game_mode = game_mode;
        self.reset_for_new_game();
        self.screen = Screen::Game;
        self.update();
    }
}
