use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} [{}]",
            self.id,
            self.description,
            if self.completed { "X" } else { " " }
        )
    }
}
