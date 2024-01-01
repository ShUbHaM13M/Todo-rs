use std::{collections::HashMap, process};

use ratatui::widgets::ListState;

use crate::{db::TodoDb, todo::Todo};

pub enum CurrentScreen {
    Main,
    AddTodo,
    DeleteTodo,
}

pub struct App {
    pub todos: HashMap<i64, Todo>,
    pub todos_db: TodoDb,
    pub should_quit: bool,
    pub current_screen: CurrentScreen,
    pub todo_input: String,
    pub selected_todo: ListState,
}

impl App {
    pub fn new() -> Self {
        if let Ok(todos_db) = TodoDb::new("todos.db") {
            let todos = todos_db.get_all_todos().unwrap_or_default();
            Self {
                todos,
                todos_db,
                should_quit: false,
                current_screen: CurrentScreen::Main,
                todo_input: String::new(),
                selected_todo: ListState::default(),
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

    fn get_selected_todo_id(&self) -> Option<i64> {
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
}
