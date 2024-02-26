use crate::models::article::Article;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ArticleListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<Article>,
}
