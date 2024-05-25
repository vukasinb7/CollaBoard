use std::sync::{Arc, Mutex};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, Router};
use axum::middleware;
use axum::routing::get_service;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::json;
use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::db::establish_connection;
pub use self::error::{Error};

#[macro_use]
extern crate diesel;
mod error;
mod routes;
mod model;
mod utils;
mod ctx;

mod db;
mod schema;
mod handlers;

pub type DbPool=Pool<ConnectionManager<PgConnection>>;


#[tokio::main]
async fn main()->Result<(),Error> {
    let db_pool = establish_connection();
    let app = Router::new()
        .merge(routes::routes_user::routes())
        .route_layer(middleware::from_fn(routes::mw_auth::guard))
        .merge(routes::routes_login::routes())
        .layer(Extension(db_pool))
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
            "error":{
                "type": client_error.as_ref(),
                "req_uuid":uuid.to_string(),

            }
        });
            println!("   ->> client_error_body:{client_error_body}");
            (*status_code,Json(client_error_body)).into_response()
        });

    println!("   ->> server log line - {uuid} - Error: {service_error:?}");
    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}