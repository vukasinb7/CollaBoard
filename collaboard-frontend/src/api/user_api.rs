use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct AuthResponse {
    pub token: String,
    pub email: String
}
pub async fn login(credentials:&str) -> AuthResponse {
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
