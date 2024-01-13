use std::{collections::HashMap, process};

use ratatui::widgets::{ListState, ScrollbarState};

use crate::{db::TodoDb, todo::Todo};

pub enum CurrentScreen {
    Main,
    AddTodo,
    DeleteTodo,
    EditTodo,
    Selection,
    Search,
}

pub struct App {
    pub todos: HashMap<i64, Todo>,
    pub todos_db: TodoDb,
    pub should_quit: bool,
    pub current_screen: CurrentScreen,
    pub todo_input: String,
    pub selected_todo: ListState,
    pub scroll_state: ScrollbarState,
    // TODO
    pub search_query: String,
}

impl App {
    pub fn new() -> Self {
        if let Ok(todos_db) = TodoDb::new("todos.db") {
            let todos = todos_db.get_all_todos().unwrap_or_default();
            let todos_len = todos.len();
            Self {
                todos,
                todos_db,
                should_quit: false,
                current_screen: CurrentScreen::Main,
                todo_input: String::new(),
                selected_todo: ListState::default(),
                scroll_state: ScrollbarState::new(todos_len).position(0),
                search_query: String::new(),
            }
        } else {
            process::exit(1);
        }
    }

    pub fn add_todo(&mut self) {
        if let Some(inserted_id) = self.todos_db.add_todo(&self.todo_input) {
            self.todos.insert(
                inserted_id,
                Todo {
                    id: inserted_id,
                    label: self.todo_input.clone(),
                    completed: false,
                },
            );
            self.todo_input.clear();
        }
    }

    pub fn go_to_top(&mut self) {
        self.selected_todo.select(Some(0));
        self.scroll_state.first();
    }

    pub fn go_to_bottom(&mut self) {
        self.selected_todo.select(Some(self.todos.len() - 1));
        self.scroll_state.last();
    }

    pub fn select_next_todo(&mut self) {
        let i = match self.selected_todo.selected() {
            Some(i) => {
                if i >= self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_todo.select(Some(i));
        if i == 0 {
            self.scroll_state.first();
            return;
        }
        self.scroll_state.next();
    }

    pub fn select_prev_todo(&mut self) {
        let i = match self.selected_todo.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_todo.select(Some(i));
        if i == self.todos.len() - 1 {
            self.scroll_state.last();
            return;
        }
        self.scroll_state.prev();
    }

    pub fn toggle_selected_todo(&mut self) {
        if let Some(id) = self.get_selected_todo_id() {
            if let Some(todo) = self.todos.get_mut(&id) {
                todo.toggle();
                self.todos_db.toggle_todo(todo.id, todo.completed);
            }
        };
    }

    pub fn get_selected_todo(&self) -> Option<&Todo> {
        if let Some(id) = self.get_selected_todo_id() {
            return Some(self.todos.get(&id).unwrap());
        }
        None
    }

    pub fn get_selected_todo_id(&self) -> Option<i64> {
        let selected_index = match self.selected_todo.selected() {
            Some(index) => index,
            None => return None,
        };

        let id = match self.todos.keys().nth(selected_index) {
            Some(key) => key.clone(),
            None => {
                eprintln!("Invalid index: {}", selected_index);
                return None;
            }
        };

        return Some(id);
    }

    pub fn delete_selected_todo(&mut self) {
        if let Some(id) = self.get_selected_todo_id() {
            if let Some(todo) = self.todos.remove(&id) {
                self.todos_db.delete_todo(todo.id);
            };
        }
    }

    pub fn update_selected_todo(&mut self) {
        if let Some(id) = self.get_selected_todo_id() {
            if let Some(todo) = self.todos.get_mut(&id) {
                todo.label = self.todo_input.clone();
                self.todo_input.clear();
                self.todos_db.update_todo(id, &todo.label.as_str());
            }
        };
    }
}
