use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
}
