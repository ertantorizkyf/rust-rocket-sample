use serde::Serialize;

#[derive(Serialize)]
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
