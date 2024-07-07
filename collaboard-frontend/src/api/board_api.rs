use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::api::permission_api::ApiResult;
use crate::api::user_api::AuthResponse;
use crate::components::board_card::BoardCardResponse;


#[derive(Serialize, Deserialize,Clone,PartialEq)]
pub struct BoardResponse {
    pub(crate) id:i32,
    pub(crate) name:String,
    pub(crate) path:String,
    pub(crate) owner_id:i32
}

#[derive( Serialize,Deserialize,Clone,PartialEq)]
pub struct SingleBoardResponse {
    pub id:i32,
    pub name:String,
    pub data:String,
    pub role:String
}
pub async fn add_board(board: &str,token: &str) -> i32 {
    let status = Request::post("http://localhost:3000/api/board")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(board.to_string())
        .send()
        .await
        .unwrap().status();

    status as i32
}

pub async fn get_my_boards(token: &str) -> Vec<BoardCardResponse> {
    Request::get("http://localhost:3000/api/board")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<Vec<BoardCardResponse>>()
        .await
        .unwrap()

}

pub async fn get_board(id:i32,token: &str) -> SingleBoardResponse {
    Request::get(format!("http://localhost:3000/api/board/{}",id).as_str())
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<SingleBoardResponse>()
        .await
        .unwrap()

}

pub async fn delete_board(board_id:i32,token: &str) -> ApiResult{
    Request::delete(&format!("http://localhost:3000/api/board/{}",board_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<ApiResult>()
        .await
        .unwrap()

}