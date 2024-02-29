use crate::responses::general::GeneralResponse;

use super::*;

use std::env;
use mysql::serde_json::from_str;
use rocket::{
    http::Status, 
    local::blocking::Client
};

const ENV_IMPL_ENDPOINT: &str = "/api/general/env-impl";

#[test]
fn rust_env_unset() {
    // api: /api/general/env-impl
    // env impl success, RUST_ENV is not set
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(ENV_IMPL_ENDPOINT).dispatch();
    
    // Check status code
    assert_eq!(response.status(), Status::Ok);
    // Check body
    let resp_body: GeneralResponse = from_str(response.into_string().unwrap().as_str()).unwrap();
    assert!(resp_body.message.contains("UNSET"));
}

#[test]
fn rust_env_set() {
    // api: /api/general/env-impl
    // env impl success, RUST_ENV is set
    env::set_var("RUST_ENV", "test");

    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(ENV_IMPL_ENDPOINT).dispatch();
    
    // Check status code
    assert_eq!(response.status(), Status::Ok);
    // Check body
    let resp_body: GeneralResponse = from_str(response.into_string().unwrap().as_str()).unwrap();
    assert!(resp_body.message.contains("UNSET"));
}
