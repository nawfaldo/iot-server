use actix_web::{dev, web, FromRequest, HttpRequest};
use bcrypt::{hash, verify, BcryptResult};
use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::errors::CustomError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

pub fn hash_password(password: &str) -> BcryptResult<String> {
    hash(password, 10)
}

pub fn verify_password(password: &str, hashed_password: &str) -> BcryptResult<bool> {
    verify(password, hashed_password)
}

pub fn generate_jwt(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let expiration = Utc::now() + Duration::days(1);

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp(),
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());

    encode(&header, &claims, &encoding_key)
}

pub struct Auth {
    pub user_id: i32,
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(token, &decoding_key, &validation)
}

impl FromRequest for Auth {
    type Error = CustomError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if auth_header.is_none() {
            return err(CustomError::Unauthorized);
        }

        let auth_header = auth_header.unwrap().to_str();
        if auth_header.is_err() {
            return err(CustomError::Unauthorized);
        }

        let auth_header = auth_header.unwrap();
        if !auth_header.starts_with("Bearer ") {
            return err(CustomError::Unauthorized);
        }

        let token = &auth_header[7..];

        match verify_jwt(token) {
            Ok(token_data) => ok(Auth {
                user_id: token_data.claims.sub,
            }),
            Err(_) => err(CustomError::Unauthorized),
        }
    }
}
