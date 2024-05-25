use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::schema::*;

#[derive(Insertable,Debug,Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}


#[derive(Identifiable,Debug, Queryable, AsChangeset,Clone)]
#[primary_key(id)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "boards"]
pub struct NewBoard {
    pub name: String,
    pub path:  String,
    pub owner_id: i32,
}

#[derive(Identifiable,Debug, Queryable, AsChangeset)]
#[primary_key(id)]
#[diesel(belongs_to(User))]
#[table_name = "boards"]
pub struct Board {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[table_name = "permissions"]
pub struct NewPermission<'a> {
    pub board_id: &'a i32,
    pub user_id: &'a i32,
    pub role: &'a i32,
}

#[derive(Identifiable,Debug, Queryable, AsChangeset)]
#[primary_key(board_id,user_id)]
#[diesel(belongs_to(Board,foreign_key=board_id))]
#[diesel(belongs_to(User,foreign_key=user_id))]
#[table_name = "permissions"]
pub struct Permission {
    pub board_id: i32,
    pub user_id: i32,
    pub role: i32,
}

#[derive(Insertable)]
#[table_name = "invitations"]
pub struct NewInvitation<'a> {
    pub code: &'a str,
    pub board_id: &'a i32,
    pub user_id: &'a i32,
    pub role: &'a i32,
    pub expire: &'a NaiveDateTime,
}

#[derive(Identifiable,Debug, Queryable, AsChangeset)]
#[primary_key(id)]
#[diesel(belongs_to(Board,foreign_key=board_id))]
#[diesel(belongs_to(User,foreign_key=user_id))]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: i32,
    pub code: String,
    pub board_id:i32,
    pub user_id:i32,
    pub role:i32,
    pub expire:NaiveDateTime
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}



