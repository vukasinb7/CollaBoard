#[macro_use]
extern crate diesel;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use axum::{Extension, Router};
use axum::middleware;
use axum::routing::{get, get_service};
use diesel::{PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::handlers::auth_handlers::who_am_i;
use crate::handlers::ws_handlers::handler;
use crate::utils::db::establish_connection;

pub use self::error::Error;

mod error;
mod routes;
mod model;
mod utils;
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
        .route("/api/whoami", get(who_am_i))
        .route_layer(middleware::from_fn(routes::mw_auth::guard))
        .merge(routes::auth_routes::routes())
        .route("/ws", get(handler))
        .layer(Extension(db_pool))
        .layer(Extension(app_state.clone()))
        .fallback_service(routes_static())
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:8080".parse::<http::HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}