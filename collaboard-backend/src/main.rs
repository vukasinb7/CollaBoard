use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, Router};
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::middleware;
use axum::routing::{get, get_service};
use axum_extra::headers::Origin;
use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use futures::{SinkExt, StreamExt};
use serde::de::Unexpected::Str;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::utils::db::establish_connection;
use crate::handlers::auth_handlers::who_am_i;
use crate::handlers::ws_handlers::handler;
use crate::model::Board;
use crate::schema::boards;
pub use self::error::{Error};

#[macro_use]
extern crate diesel;
mod error;
mod routes;
mod model;
mod utils;
mod ctx;

mod schema;
mod handlers;
mod dto;

pub type DbPool=Pool<ConnectionManager<PgConnection>>;

struct WSState {
    rooms: Mutex<HashMap<String, RoomState>>,
}
struct RoomState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
    buff:Mutex<HashMap<String,String>>,
}

impl RoomState {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashSet::new()),
            tx: broadcast::channel(69).0,
            buff:Mutex::new(HashMap::new())
        }
    }
}


#[tokio::main]
async fn main()->Result<(),Error> {
    let db_pool = establish_connection();
    let app_state = Arc::new(WSState {
        rooms: Mutex::new(HashMap::new())
    });
    let app = Router::new()
        .merge(routes::permission_routes::routes())
        .merge(routes::board_routes::routes())
        .route("/whoami", get(who_am_i))
        .route_layer(middleware::from_fn(routes::mw_auth::guard))
        .merge(routes::auth_routes::routes())
        .route("/ws", get(handler))
        .layer(Extension(db_pool))
        .layer(Extension(app_state.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any) // You can specify methods, e.g., vec![Method::GET, Method::POST]
                .allow_headers(Any) // You can specify headers, e.g., vec![header::AUTHORIZATION, header::ACCEPT]
        );;

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