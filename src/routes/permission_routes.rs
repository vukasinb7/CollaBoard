use axum::Router;
use axum::routing::{put, post};
use crate::handlers::permission_handlers::{accept_invitation, change_permission, create_invitation, delete_permission};

pub fn routes() -> Router {
    Router::new()
        .route("/api/invite", post(create_invitation))
        .route("/api/invite/accept/:invitation_code", put(accept_invitation))
        .route("/api/permission", put(change_permission).delete(delete_permission))
}
