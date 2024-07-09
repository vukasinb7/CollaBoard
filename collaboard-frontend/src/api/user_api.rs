use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

use crate::api::permission_api::ApiResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub email: String,
}

pub async fn login(credentials: &str) -> Result<AuthResponse, String> {
    let response = Request::post(&format!("{}/login", "http://localhost:3000/api"))
        .header("content-type", "application/json")
        .body(credentials)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == 404 {
                Err("Invalid credentials".to_string())
            } else if res.ok() {
                match res.json::<AuthResponse>().await {
                    Ok(auth_response) => Ok(auth_response),
                    Err(_) => Err("Something went wrong".to_string()),
                }
            } else {
                Err("Something went wrong".to_string())
            }
        },
        Err(_) => Err("Something went wrong".to_string()),
    }
}

pub async fn register(credentials: &str) -> Result<ApiResult, String> {
    let response = Request::post(&format!("{}/register", "http://localhost:3000/api"))
        .header("content-type", "application/json")
        .body(credentials)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == 404 {
                Err("Something went wrong".to_string())
            }else if res.status() == 400 {
                Err("Email already exists".to_string())
            } else if res.ok() {
                match res.json::<ApiResult>().await {
                    Ok(api_result) => Ok(api_result),
                    Err(_) =>Err("Something went wrong".to_string())
                }
            } else {
                Err("Something went wrong".to_string())
            }
        },
        Err(_) => Err("Network error occurred".to_string()),
    }
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
