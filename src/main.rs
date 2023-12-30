pub mod app;
pub mod db;
pub mod todo;

use app::App;
use rusqlite::Result;
use todo::Todo;

fn main() -> Result<()> {
    let app = App::new();

    let query = "SELECT * FROM todos";
    let mut stmt = app.todos_db.conn.prepare(&query)?;
    let todos = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            label: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;
    let todos: Vec<Todo> = todos.map(|x| x.unwrap()).collect();

    if todos.len() == 0 {
        println!("Nothing left todo 😎");
    } else {
        for todo in todos {
            println!("Todo: {:?}", todo);
        }
    }

    Ok(())
}
