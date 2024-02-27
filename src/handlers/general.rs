use crate::{
    helpers::calc::calculate_fibonacci,
    responses::general::{ GeneralResponse, GeneralResponseWithData }
};

use std::{
    env, 
    time::Instant
};
use rocket::{
    get, http::Status, serde::json::Json 
};

#[get("/env-impl")]
pub fn env_impl() -> Result<Json<GeneralResponse>, Status> {
    let rust_env = match env::var_os("RUST_ENV") {
        Some(v) => v.into_string().unwrap(),
        None => "unset".to_string()
    };


    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: rust_env,
    };

    Ok(Json(json_response))
}

#[get("/fibonacci?<number>")]
pub fn fibonacci(number: usize) -> Result<Json<GeneralResponseWithData>, Status> {
    let now: Instant = Instant::now();
    let fib_num = calculate_fibonacci(number);

    let json_response = GeneralResponseWithData {
        status: "success".to_string(),
        message: format!(
            "{}s and {}ns",
            now.elapsed().as_secs(), now.elapsed().as_nanos()
        ),
        data: fib_num.to_string()
    };

    Ok(Json(json_response))
}

#[get("/middleware-impl-test")]
pub fn middleware_impl_test() -> Result<Json<GeneralResponse>, Status> {
    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: "middleware impl testing".to_string()
    };

    Ok(Json(json_response))
}
