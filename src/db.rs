use std::collections::HashMap;

use rusqlite::{Connection, Result};

use crate::todo::Todo;

pub struct TodoDb {
    pub conn: Connection,
}

impl TodoDb {
    pub fn new(url: &str) -> Result<Self> {
        let conn = Connection::open(url)?;

        let query = "
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                label TEXT NOT NULL,
                completed BOOLEAN NOT NULL CHECK (completed IN (0, 1))
            )";

        match conn.execute(&query, ()) {
            Ok(_) => println!("Successfully created todos table"),
            Err(err) => eprintln!("Error: unable to create todos table {}", err),
        };

        Ok(Self { conn })
    }

    pub fn get_all_todos(&self) -> Result<HashMap<i64, Todo>> {
        let query = "SELECT * FROM todos";
        let mut stmt = self.conn.prepare(&query)?;
        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                label: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        let todos: HashMap<i64, Todo> = todos
            .map(|todo| {
                let todo = todo.unwrap();
                (todo.id, todo)
            })
            .collect();

        Ok(todos)
    }

    pub fn add_todo(&self, todo: &str) -> Option<i64> {
        let query = "INSERT INTO todos (label, completed) VALUES(?1, ?2)";
        if let Ok(mut stmt) = self.conn.prepare(query) {
            let res = stmt.execute([todo, "0"]);
            if res.is_ok() {
                return Some(self.conn.last_insert_rowid());
            }
        };
        None
    }

    pub fn toggle_todo(&self, id: i64, completed: bool) {
        let query = "
            UPDATE todos
            SET completed = (?1)
            WHERE
                id = (?2)
        ";
        if let Ok(mut stmt) = self.conn.prepare(query) {
            let _ = stmt.execute([
                {
                    if completed {
                        1
                    } else {
                        0
                    }
                },
                id,
            ]);
        }
    }

    pub fn update_todo(&self, id: u64) {}
}
