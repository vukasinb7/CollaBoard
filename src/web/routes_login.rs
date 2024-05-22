use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{Error};
use crate::utils::jwt::encode_jwt;

pub fn routes() -> Router {
    Router::new().route("/api/login",post(login))
}

async fn login(payload:Json<LoginPayload>)->Result<Json<Value>,Error>{
    println!("--> {:<12} - login","HANDLER");

    //TODO: implement DB Check data
    if payload.email!="admin" || payload.password !="admin"{
        return Err(Error::LoginFail)
    }
    let token=encode_jwt(payload.email.clone())
        .map_err(|_|Error::LoginFail)?;

    let body = Json(json!({
        "result":{"token":token}
    }));
    Ok(body)
}

#[derive(Debug,Deserialize)]
struct LoginPayload{
    email:String,
    password:String
}