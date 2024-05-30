use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Deserialize)]
pub struct BoardPayload {
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct UserResponse{
    pub name:String,
    pub surname:String,
    pub email:String
}

#[derive(Debug,Deserialize)]
pub struct InvitationPayload{
    pub user_email:String,
    pub board_id:i32,
    pub role:i32
}
#[derive(Debug, Deserialize)]
pub struct DeletePermissionParams{
    pub user_email:String,
    pub board_id:i32
}



