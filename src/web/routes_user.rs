use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use crate::Error;
use crate::model::{ModelController, User, UserDto};

#[derive(Clone,FromRef)]
struct AppState{
    mc:ModelController
}

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(list_users))
}


async fn list_users(
)->Result<String,Error>{
    println!("->>{:<12} - list-users","HANDLE");
    Ok("Uspeo".to_string())
}



async fn delete_user(
    State(mc):State<ModelController>,
    Path(id):Path<u64>
)-> Result<Json<User>,Error>{
    println!(">>> {:<15} - delete_user","HANDLER");
    let user=mc.delete_user(id).await?;
    Ok(Json(user))
}