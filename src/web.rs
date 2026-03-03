use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::app::{App, Screen};
use crate::draw::{render_game, render_game_menu, render_help, render_start_screen, render_victory};
use crate::game_state::GameMode;
use crate::selection::Selection;
use ratzilla::event::KeyCode;
use ratzilla::ratatui::Terminal;
use ratzilla::{DomBackend, WebRenderer};

pub fn run() -> io::Result<()> {
    let app = Rc::new(RefCell::new(App::new()));

    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let app = app.clone();
        move |event| {
            let mut app = app.borrow_mut();

            if app.screen == Screen::Quit {
                app.screen = Screen::Start;
            }

            match app.screen {
                Screen::Start => match event.code {
                    KeyCode::Char('1') => app.new_game(GameMode::DrawOne),
                    KeyCode::Char('3') => app.new_game(GameMode::DrawThree),
                    _ => {}
                },
                Screen::Game => {
                    match event.code {
                        KeyCode::Left => app.cursor.move_left(),
                        KeyCode::Right => app.cursor.move_right(),
                        KeyCode::Up => app.cursor.select_up(),
                        KeyCode::Down => app.cursor.select_down(),
                        KeyCode::Home => app.cursor = Selection::Deck,
                        KeyCode::End => app.cursor = Selection::Pile { index: 0 },
                        KeyCode::Char(' ') => app.cards_action(),
                        KeyCode::Enter => app.enter_key_action(),
                        KeyCode::Char('c') if app.debug_mode => {
                            app.debug_unchecked_cards_action()
                        }
                        KeyCode::Char('x') => app.selected = None,
                        KeyCode::Char('z') if app.debug_mode => app.debug_check_valid(),
                        KeyCode::Char('d') => app.debug_mode = !app.debug_mode,
                        KeyCode::Char('h') => app.screen = Screen::Help,
                        KeyCode::Esc => app.screen = Screen::GameMenu,
                        _ => {}
                    }
                    app.update();
                }
                Screen::GameMenu => match event.code {
                    KeyCode::Char('1') => app.new_game(GameMode::DrawOne),
                    KeyCode::Char('3') => app.new_game(GameMode::DrawThree),
                    KeyCode::Char('r') => app.restart_game(),
                    KeyCode::Char('q') => app.screen = Screen::Start,
                    KeyCode::Esc => app.screen = Screen::Game,
                    _ => {}
                },
                Screen::Help => {
                    app.screen = Screen::Game;
                }
                Screen::Victory => match event.code {
                    KeyCode::Char('y') => {
                        let mode = app.game_state.game_mode;
                        app.new_game(mode);
                    }
                    KeyCode::Char('n') | KeyCode::Esc => app.screen = Screen::Start,
                    _ => {}
                },
                Screen::Quit => {}
            }
        }
    });

    terminal.draw_web({
        let app = app.clone();
        move |frame| {
            let app = app.borrow();
            match app.screen {
                Screen::Start | Screen::Quit => render_start_screen(frame),
                Screen::Game => render_game(frame, &app),
                Screen::GameMenu => render_game_menu(frame, &app),
                Screen::Help => render_help(frame, &app),
                Screen::Victory => render_victory(frame, &app),
            }
        }
    });

    Ok(())
}
