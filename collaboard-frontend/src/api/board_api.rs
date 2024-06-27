use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use crate::api::user_api::AuthResponse;


#[derive(Serialize, Deserialize,Clone,PartialEq)]
pub struct BoardResponse {
    pub(crate) id:i32,
    pub(crate) name:String,
    pub(crate) path:String,
    pub(crate) owner_id:i32
}
pub async fn add_board(board: &str,token: &str) -> i32 {
    let response = Request::post("http://localhost:3000/api/board")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(board.to_string())
        .send()
        .await
        .unwrap()
        .json::<BoardResponse>()
        .await
        .unwrap();

    200
}

pub async fn get_my_boards(token: &str) -> Vec<BoardResponse> {
    Request::get("http://localhost:3000/api/board")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<Vec<BoardResponse>>()
        .await
        .unwrap()

}