use chrono::NaiveDateTime;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone,PartialEq)]
pub struct InvitationPayload {
    pub id: i32,
    pub code: String,
    pub role:i32,
    pub expire:NaiveDateTime,
    pub user_id:i32,
    pub board_id:i32,
}

#[derive(Serialize, Deserialize,Clone,PartialEq)]
pub struct PermissionPayload{
    pub email: String,
    pub permission_type:i32,
}
#[derive(Serialize, Deserialize,Clone,PartialEq)]
pub struct ApiResult{
    success:bool
}

pub async fn invite_user(form_data: &str,token: &str) -> bool {
    let status = Request::post("http://localhost:3000/api/invite")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .body(form_data.to_string())
        .send()
        .await
        .unwrap().status();

    status==200
}

pub async fn get_board_permissions(board_id:i32,token: &str) -> Vec<PermissionPayload> {
    Request::get(&format!("http://localhost:3000/api/permission/{}",board_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<Vec<PermissionPayload>>()
        .await
        .unwrap()

}



pub async fn delete_permission(user_email:String,board_id:i32,token: &str) -> ApiResult{
    Request::delete(&format!("http://localhost:3000/api/permission?user_email={}&board_id={}",user_email,board_id))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .json::<ApiResult>()
        .await
        .unwrap()

}

pub async fn accept_permission(code:String,token: &str) -> bool{
    let status=Request::put(&format!("http://localhost:3000/api/invite/accept/{}",code))
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .unwrap()
        .status();

    status==200
}