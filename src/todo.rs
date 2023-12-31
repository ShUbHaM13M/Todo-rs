#[derive(Debug)]
pub struct Todo {
    pub id: i64,
    pub label: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i64, label: String) -> Self {
        Self {
            id,
            label,
            completed: false,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}
