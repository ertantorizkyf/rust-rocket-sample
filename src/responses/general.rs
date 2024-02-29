use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GeneralResponse {
    pub status: String,
    pub message: String
}

#[derive(Serialize)]
pub struct GeneralResponseWithData {
    pub status: String,
    pub message: String,
    pub data: String
}
