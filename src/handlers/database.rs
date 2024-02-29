use crate::{
    constants::database::TBL_ARTICLES,
    models::article::Article,
    helpers::database::get_conn,
    responses::{article::ArticleListResponse, general::GeneralResponse}
};

use mysql::*;
use mysql::prelude::*;
use rocket::{
    get, 
    http::Status, 
    post, 
    response::status::Custom,
    serde::json::{
        to_pretty_string, 
        Json
    }
};

#[get("/articles?<page>&<limit>")]
pub async fn get_articles(
    page: Option<usize>,
    limit: Option<usize>
) -> Result<Json<ArticleListResponse>, Status> {
    let limit = limit.unwrap_or(20);
    let page = (page.unwrap_or(1) - 1) * limit;

    let table_name = TBL_ARTICLES;
    let mut conn = get_conn();
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

    let articles = tx
        .query_map(
            format!("SELECT * FROM {} LIMIT {} OFFSET {}", table_name, limit.to_string(), page.to_string()).as_str(),
            |(id, title, content)| {
                Article { id, title, content }
            },
        ).unwrap();
    let _ = tx.commit();

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
) -> Result<Json<GeneralResponse>, Custom<Json<GeneralResponse>>> {
    let article = Article {
        id: Some(0),
        title: data.title.clone(),
        content: data.content.clone()
    };

    let table_name = TBL_ARTICLES;
    let mut conn = get_conn();
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
    
    let query_result = tx
        .exec_drop(
            format!("INSERT INTO {} (title, content) VALUES (?, ?)", table_name).as_str(),
            (&data.title.to_owned(), &data.content.to_owned()),
        );

    let mut is_error = false;
    let mut error_msg = String::new();
    let _ = match query_result {
        Ok(()) => { let _ = tx.commit(); },
        Err(e) => { 
            is_error = true;
            let _ = tx.rollback(); 
            error_msg = e.to_string();
        }
    };

    if is_error {
        let error_response = GeneralResponse {
            status: "failed".to_string(),
            message: error_msg
        };

        return Err(Custom(Status::InternalServerError, Json(error_response)));
    }

    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: to_pretty_string(&article).expect("LogRocket: error parsing to JSON")
    };

    Ok(Json(json_response))
}
