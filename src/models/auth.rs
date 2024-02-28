use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthBody {
    pub password: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtClaims {
    pub name: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize
}
