use crate::app::{App, Screen};
use crate::draw::Draw;
use crate::game_state::GameMode;
use crate::selection::Selection;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub struct Ui {
    app: App,
    draw: Draw,
}

impl Ui {
    pub fn new(app: App) -> Self {
        Self {
            app,
            draw: Draw::new(),
        }
    }

    fn read_key_event() -> event::KeyEvent {
        loop {
            if let Ok(Event::Key(key)) = event::read()
                && key.kind == KeyEventKind::Press
            {
                return key;
            }
        }
    }

    fn run_game(&mut self) {
        self.app.update();
        if self.app.screen != Screen::Game {
            return;
        }
        self.draw.display_game_state(&self.app);

        loop {
            let key = Self::read_key_event();
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                KeyCode::Left => self.app.cursor.move_left(),
                KeyCode::Right => self.app.cursor.move_right(),
                KeyCode::Up => self.app.cursor.select_up(),
                KeyCode::Down => self.app.cursor.select_down(),
                KeyCode::Home => self.app.cursor = Selection::Deck,
                KeyCode::End => self.app.cursor = Selection::Pile { index: 0 },
                KeyCode::Char(' ') => self.app.cards_action(),
                KeyCode::Enter => self.app.enter_key_action(),
                KeyCode::Char('c') if self.app.debug_mode => {
                    self.app.debug_unchecked_cards_action()
                }
                KeyCode::Char('x') => self.app.selected = None,
                KeyCode::Char('z') if self.app.debug_mode => self.app.debug_check_valid(),
                KeyCode::Char('d') => self.app.debug_mode = !self.app.debug_mode,
                KeyCode::Char('h') => {
                    self.app.screen = Screen::Help;
                    break;
                }
                KeyCode::Esc => {
                    self.app.screen = Screen::GameMenu;
                    break;
                }
                _ => {}
            }
            self.app.update();
            if self.app.screen != Screen::Game {
                return;
            }
            self.draw.display_game_state(&self.app);
        }
    }

    fn run_start_screen(&mut self) {
        self.draw.display_start_screen();
        loop {
            let key = Self::read_key_event();
            match key.code {
                KeyCode::Char('1') => {
                    self.app.new_game(GameMode::DrawOne);
                    break;
                }
                KeyCode::Char('3') => {
                    self.app.new_game(GameMode::DrawThree);
                    break;
                }
                KeyCode::Esc | KeyCode::Char('c')
                    if key.code == KeyCode::Esc
                        || key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    self.app.screen = Screen::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    fn run_game_menu(&mut self) {
        self.draw.display_game_menu(&self.app);
        loop {
            let key = Self::read_key_event();
            match key.code {
                KeyCode::Char('1') => {
                    self.app.new_game(GameMode::DrawOne);
                    break;
                }
                KeyCode::Char('3') => {
                    self.app.new_game(GameMode::DrawThree);
                    break;
                }
                KeyCode::Char('r') => {
                    self.app.restart_game();
                    break;
                }
                KeyCode::Char('q') => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                KeyCode::Esc => {
                    self.app.screen = Screen::Game;
                    break;
                }
                _ => {}
            }
        }
    }

    fn run_victory(&mut self) {
        self.draw.display_victory(&self.app);
        loop {
            let key = Self::read_key_event();
            match key.code {
                KeyCode::Char('y') => {
                    self.app.new_game(self.app.game_state.game_mode);
                    break;
                }
                KeyCode::Char('n') | KeyCode::Esc => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    fn run_help(&mut self) {
        self.draw.display_help(&self.app);
        Self::read_key_event();
        self.app.screen = Screen::Game;
    }

    pub fn run(&mut self) {
        loop {
            match self.app.screen {
                Screen::Start => self.run_start_screen(),
                Screen::Game => self.run_game(),
                Screen::GameMenu => self.run_game_menu(),
                Screen::Help => self.run_help(),
                Screen::Victory => self.run_victory(),
                Screen::Quit => break,
            }
        }

        self.draw.restore();
        println!("please send bug reports via IRC or ham radio");
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::Card;
    use crate::game_state::GameState;
    use crate::selection::Selection;

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
