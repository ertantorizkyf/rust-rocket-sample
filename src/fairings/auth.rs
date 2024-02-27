use crate::{
    constants::{
        auth::DEFAULT_JWT_SECRET,
        fairings::JWT_VALIDATION_FAIRING_WHITELIST
    },
    models::auth::JwtClaims
};

use std::{
    borrow::Borrow,
    env,
    ops::Index
};
use rocket::{
    fairing::{
        Fairing, 
        Info, 
        Kind
    }, http::Header, Data, Request
};
use jsonwebtoken::{
    decode, DecodingKey, TokenData, Validation
};

pub struct JwtValidationFairing;

#[rocket::async_trait]
impl Fairing for JwtValidationFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT Validation Fairing",
            kind: Kind::Ignite | Kind::Request
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        // only implement this fairing to specific endpoint
        let current_uri = req.uri().to_string();
        if JWT_VALIDATION_FAIRING_WHITELIST.contains(&current_uri.borrow()) {
            let authorization = req.headers().get_one("authorization").unwrap_or(""); 
            if authorization != "" {
                let split_authorization: Vec<&str> = authorization.split_whitespace().collect();
                if split_authorization.len() == 2 {
                    let bearer_token = split_authorization.index(1);
                    let jwt_secret = match env::var_os("JWT_SECRET") {
                        Some(v) => v.into_string().unwrap(),
                        None => DEFAULT_JWT_SECRET.to_string()
                    };
                    
                    let decoded_token = decode::<JwtClaims>(&bearer_token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::default()).unwrap_or(TokenData { 
                        header: jsonwebtoken::Header::default(), 
                        claims: JwtClaims { 
                            name: "".to_string(), 
                            email: "".to_string(), 
                            exp: 0, 
                            iat: 0
                        }
                    });

                    req.add_header(Header::new("name", decoded_token.claims.name.to_string()));
                    req.add_header(Header::new("email", decoded_token.claims.email.to_string()));
                }
            }
        }
    }
}
