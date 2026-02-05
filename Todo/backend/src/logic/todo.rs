use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    id: u32,
    task: String,
    completed: bool,
}
