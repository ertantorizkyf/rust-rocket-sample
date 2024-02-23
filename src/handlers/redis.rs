use crate::{
    models::{
        general::GeneralBody,
        redis::RustRocketRedis
    },
    responses::{
        general::GeneralResponse,
        redis::RedisHashResponse
    }
};

use redis::Commands;
use rocket::{
    get, http::Status, post, serde::json::Json
};

#[get("/")]
pub fn redis_get() -> Result<Json<GeneralResponse>, Status> {
    let mut client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let redis_val: String = client.get("rust_rocket").unwrap_or("NIL".to_string());
    
    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: redis_val
    };

    Ok(Json(json_response))
}

#[post("/", data = "<body>")]
pub fn redis_set(
    body: Json<GeneralBody>
) -> Result<Json<GeneralResponse>, Status> {
    let redis_val = &body.content;

    let mut client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let _: () = client.set("rust_rocket", redis_val).unwrap();

    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: format!("{} inserted to redis with key rust_rocket", redis_val)
    };

    Ok(Json(json_response))
}

#[get("/hash")]
pub fn redis_get_hash() -> Result<Json<RedisHashResponse>, Status> {
    let mut client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let redis_val: Vec<String> = client.hgetall("rust_rocket_hash").unwrap_or(Vec::new());

    let mut hash_data = Vec::<RustRocketRedis>::new();
    for i in (0..redis_val.len()).step_by(2) {
        let current_hash = RustRocketRedis {
            field: redis_val[i].to_owned(),
            value: redis_val[i + 1].to_owned()
        };

        hash_data.push(current_hash);
    }
    
    let json_response = RedisHashResponse {
        status: "success".to_string(),
        data: hash_data
    };

    Ok(Json(json_response))
}

#[post("/hash", data = "<body>")]
pub fn redis_set_hash(
    body: Json<RustRocketRedis>
) -> Result<Json<GeneralResponse>, Status> {
    let field = &body.field;
    let value = &body.value;

    let mut client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let _: () = client.hset("rust_rocket_hash", field, value).unwrap();

    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: format!("field {} with value {} inserted to redis with key rust_rocket_hash", field, value)
    };

    Ok(Json(json_response))
}
