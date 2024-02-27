use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub status: String,
    pub message: String,
    pub token: String
}
