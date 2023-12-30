use rusqlite::{Connection, Result};

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
}
