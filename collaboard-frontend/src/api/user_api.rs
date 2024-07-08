use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

use crate::api::permission_api::ApiResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub email: String,
}

pub async fn login(credentials: &str) -> AuthResponse {
    Request::post(&format!("{}/login", "http://localhost:3000/api"))
        .header("content-type", "application/json")
        .body(
            credentials
        )
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await
        .unwrap()
}

pub async fn register(credentials: &str) -> ApiResult {
    Request::post(&format!("{}/register", "http://localhost:3000/api"))
        .header("content-type", "application/json")
        .body(
            credentials
        )
        .send()
        .await
        .unwrap()
        .json::<ApiResult>()
        .await
        .unwrap()
}

pub async fn whoami(token: &str) -> i32 {
    let response = Request::get("http://localhost:3000/api/whoami")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap().status();

    response as i32
}
