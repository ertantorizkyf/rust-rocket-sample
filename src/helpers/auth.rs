use crate::{
    constants::auth::DEFAULT_JWT_SECRET,
    models::auth::JwtClaims
};

use::std::env;
use std::ops::Add;
use chrono::Utc;
use jsonwebtoken::{encode, Header, EncodingKey};

pub fn generate_token(name: String, email: String) -> String {
    let jwt_secret = match env::var_os("JWT_SECRET") {
        Some(v) => v.into_string().unwrap(),
        None => DEFAULT_JWT_SECRET.to_string()
    };
    let now = Utc::now()
        .timestamp();
    let expiration = Utc::now()
        .add(chrono::Duration::hours(24))
        .timestamp();

    let jwt_claims = JwtClaims {
        name,
        email,
        iat: usize::try_from(now).unwrap_or(0),
        exp: usize::try_from(expiration).unwrap_or(0)
    };

    let token = encode(&Header::default(), &jwt_claims, &EncodingKey::from_secret(jwt_secret.as_ref())).unwrap_or("".to_string());

    token
}