use axum::http::StatusCode;
use chrono::{Utc, Duration};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::utils;

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct Ctx {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let claim = Ctx {
        iat: now.timestamp() as usize,
        exp: (now+Duration::hours(24)).timestamp() as usize,
        email,
    };

    let secret = (*utils::constants::TOKEN).clone();
    return encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| { StatusCode::UNAUTHORIZED });
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Ctx>, StatusCode> {
    let secret = (*utils::constants::TOKEN).clone();
    let res:Result<TokenData<Ctx>,StatusCode> = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|_| { StatusCode::UNAUTHORIZED });
    return res;
}
