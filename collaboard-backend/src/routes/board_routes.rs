use axum::Router;
use axum::routing::{get, post};
use crate::handlers::board_handlers::{create_board, delete_board, get_board, get_my_boards};

pub fn routes() -> Router {
    Router::new()
        .route("/api/board", post(create_board).get(get_my_boards))
        .route("/api/board/:board_id", get(get_board).delete(delete_board))
}