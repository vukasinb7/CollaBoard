use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use crate::ctx::Ctx;
use crate::Error;



pub fn routes() -> Router {
    Router::new()
        .route("/users", get(list_users))
}


async fn list_users(ctx:Ctx)->Result<String,Error>{
    println!("->>{:<12} - list-users","HANDLE");

    Ok(ctx.email.to_string())
}

