use crate::app::{App, CurrentScreen};
use crossterm::event::{KeyCode, KeyEvent};

fn handle_main_screen_events(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        KeyCode::Char('a') => {
            app.current_screen = CurrentScreen::AddTodo;
        }
        KeyCode::Char('j') => {
            app.select_next_todo();
        }
        KeyCode::Char('k') => {
            app.select_prev_todo();
        }
        KeyCode::Char(' ') => {
            app.toggle_selected_todo();
        }
        _ => {}
    }
}

fn handle_add_screen_events(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Main;
            app.todo_input.clear();
        }
        KeyCode::Backspace => {
            let _ = app.todo_input.pop();
        }
        KeyCode::Enter => {
            app.add_todo();
            app.current_screen = CurrentScreen::Main;
        }
        KeyCode::Char(c) => {
            app.todo_input.push(c);
        }
        _ => {}
    }
}

fn handle_delete_screen_events(_key: &KeyEvent) {}

pub fn update(app: &mut App, key: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => handle_main_screen_events(app, &key),
        CurrentScreen::AddTodo => handle_add_screen_events(app, &key),
        CurrentScreen::DeleteTodo => handle_delete_screen_events(&key),
    }
}
