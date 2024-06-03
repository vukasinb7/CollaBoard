use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug,Validate, Deserialize)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min=1))]
    pub password: String,
}
#[derive(Debug,Validate, Deserialize)]
pub struct BoardPayload {
    #[validate(length(min=1))]
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct UserResponse{

    pub name:String,
    pub surname:String,
    pub email:String
}

#[derive(Debug,Validate,Deserialize)]
pub struct InvitationPayload{
    #[validate(email)]
    pub user_email:String,
    #[validate(range(min = 0))]
    pub board_id:i32,
    #[validate(range(min = 0))]
    pub role:i32
}
#[derive(Debug,Validate, Deserialize)]
pub struct DeletePermissionParams{
    #[validate(email)]
    pub user_email:String,
    #[validate(range(min = 0))]
    pub board_id:i32
}


