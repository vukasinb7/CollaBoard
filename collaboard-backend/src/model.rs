use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use validator::{Validate};

#[derive(Insertable,Debug,Validate,Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min=1))]
    pub name: String,
    #[validate(length(min=1))]
    pub surname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min=1))] //TODO:Password complexity check
    pub password: String,
}


#[derive(Identifiable,Debug, Queryable, AsChangeset,Clone)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable,Debug,Deserialize)]
#[diesel(table_name = boards)]
pub struct NewBoard {
    pub name: String,
    pub path:  String,
    pub owner_id: i32,
}

#[derive(Identifiable,Debug, Queryable, AsChangeset,Serialize)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = boards)]
pub struct Board {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = permissions)]
pub struct NewPermission{
    pub board_id: i32,
    pub user_id:  i32,
    pub role: i32,
}

#[derive(Identifiable,Debug, Queryable, AsChangeset,Serialize)]
#[diesel(primary_key(board_id,user_id))]
#[diesel(belongs_to(Board,foreign_key=board_id))]
#[diesel(belongs_to(User,foreign_key=user_id))]
#[diesel(table_name = permissions)]
pub struct Permission {
    pub board_id: i32,
    pub user_id: i32,
    pub role: i32,
}

#[derive(Insertable)]
#[diesel(table_name = invitations)]
pub struct NewInvitation {
    pub code:String,
    pub role: i32,
    pub expire: NaiveDateTime,
    pub board_id: i32,
    pub user_id: i32,
}

#[derive(Identifiable,Debug, Queryable, AsChangeset,Serialize,Deserialize)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Board,foreign_key=board_id))]
#[diesel(belongs_to(User,foreign_key=user_id))]
#[diesel(table_name = invitations)]
pub struct Invitation {
    pub id: i32,
    pub code: String,
    pub role:i32,
    pub expire:NaiveDateTime,
    pub user_id:i32,
    pub board_id:i32,
}





