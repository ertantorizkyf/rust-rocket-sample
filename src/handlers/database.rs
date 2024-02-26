use crate::{
    models::article::Article,
    responses::{article::ArticleListResponse, general::GeneralResponse}
};

use mysql::*;
use mysql::prelude::*;
use rocket::{
    get, http::Status, post, serde::json::{to_pretty_string, Json}
};

#[get("/articles?<page>&<limit>")]
pub async fn get_articles(
    page: Option<usize>,
    limit: Option<usize>
) -> Result<Json<ArticleListResponse>, Status> {
    let limit = limit.unwrap_or(20);
    let page = (page.unwrap_or(1) - 1) * limit;

    let db_url = "mysql://root:@localhost/rust_db";
    let table_name = "articles";
    let pool = Pool::new(db_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let articles = conn
        .query_map(
            format!("SELECT * FROM {} LIMIT {} OFFSET {}", table_name, limit.to_string(), page.to_string()).as_str(),
            |(id, title, content)| {
                Article { id, title, content }
            },
        ).unwrap();

    let json_response = ArticleListResponse {
        status: "success".to_string(),
        results: articles.len(),
        data: articles
    };

    Ok(Json(json_response))
}

#[post("/articles", data="<data>")]
pub async fn insert_article(
    data: Json<Article>
) -> Result<Json<GeneralResponse>, Status> {
    let article = Article {
        id: Some(0),
        title: data.title.clone(),
        content: data.content.clone()
    };

    let db_url = "mysql://root:@localhost/rust_db";
    let table_name = "articles";
    let pool = Pool::new(db_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn
        .exec_drop(
            format!("INSERT INTO {} (title, content) VALUES (?, ?)", table_name).as_str(),
            (&data.title.to_owned(), &data.content.to_owned()),
        ).unwrap();

    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: to_pretty_string(&article).expect("LogRocket: error parsing to JSON")
    };

    Ok(Json(json_response))
}
