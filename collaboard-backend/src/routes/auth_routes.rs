use axum::Router;
use axum::routing::post;
use crate::handlers::auth_handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(auth_handlers::login))
        .route("/api/register", post(auth_handlers::register))
}

