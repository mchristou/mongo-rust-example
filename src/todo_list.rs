use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub title: String,
    pub description: String,
}
