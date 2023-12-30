use std::process;

use crate::db::TodoDb;

pub struct App {
    pub todos_db: TodoDb,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        if let Ok(todos_db) = TodoDb::new("todos.db") {
            Self {
                todos_db,
                should_quit: false,
            }
        } else {
            process::exit(1);
        }
    }
}
