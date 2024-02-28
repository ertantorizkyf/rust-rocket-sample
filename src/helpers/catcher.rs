use crate::responses::general::GeneralResponse;

use rocket::{
    Request,
    serde::json::Json
};

#[catch(404)]
pub fn not_found(req: &Request) -> Json<GeneralResponse> {
    let uri = req.uri().to_string();
    let response = GeneralResponse { 
        status: "failed".to_string(),
        message: format!("PAGE {} COULD NOT BE FOUND!!!", uri)
    };

    Json(response)
}
