use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Article {
    pub id: Option<usize>,
    pub title: String,
    pub content: String
}
