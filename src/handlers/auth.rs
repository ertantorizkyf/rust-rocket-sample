use crate::{
    constants::auth::{
        DUMMYPASS,
        DUMMYMAIL,
        DUMMYNAME
    },
    helpers::auth::generate_token,
    models::auth::AuthBody,
    responses::auth::AuthResponse
};

use bcrypt::verify;
use rocket::{
    post,
    http::Status,
    response::status::Custom,
    serde::json::Json
};

#[post("/", data="<body>")]
pub fn authorization(
    body: Json<AuthBody>
) -> Result<Json<AuthResponse>, Custom<Json<AuthResponse>>> {
    // let hashed_pass = hash("helloWorld", DEFAULT_COST).unwrap();
    // dummy pass const is generated using the code above
    let hashed_pass = String::from(DUMMYPASS);
    let is_matched = verify(body.password.clone(), &hashed_pass).unwrap();

    if !is_matched { 
        let err_response = AuthResponse {
            status: "failed".to_string(),
            message: "Wrong password!".to_string(),
            token: "".to_string()
        };

        return Err(Custom(Status::Unauthorized, Json(err_response)));
    } 
    
    let token = generate_token(DUMMYNAME.to_string(), DUMMYMAIL.to_string());
    let json_response = AuthResponse {
        status: "success".to_string(),
        message: "Password matched".to_string(),
        token
    };

    Ok(Json(json_response))
}
