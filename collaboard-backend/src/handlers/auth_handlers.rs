use axum::{Extension, Json};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::{json, Value};
use sha3::Digest;
use crate::{DbPool, Error};
use crate::ctx::Ctx;
use crate::dto::{LoginPayload, UserResponse};
use crate::model::{NewUser, User};
use crate::schema::users;
use crate::utils::jwt::encode_jwt;
use validator::{Validate};

pub async fn login(Extension(pool): Extension<DbPool>, payload: Json<LoginPayload>) -> Result<Json<Value>, Error> {
    payload.validate().map_err(|_|Error::BadRequest)?;

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(payload.email.clone())).first::<User>(&mut connection).map_err(|_| Error::UserNotFound)?;

    let password_hash = sha3::Sha3_256::digest(payload.password.as_bytes());
    let password_hash = format!("{:x}", password_hash);
    if user.password.eq(&password_hash.clone()) {
        let token = encode_jwt(payload.email.clone())
            .map_err(|_| Error::TokenEncodingFail)?;

        let body = Json(json!({"token":token,"email":user.email}));
        Ok(body)
    } else {
        Err(Error::UserNotFound)
    }
}

pub async fn register(Extension(pool): Extension<DbPool>, Json(mut user): Json<NewUser>) -> Result<Json<Value>, Error> {
    user.validate().map_err(|_|Error::BadRequest)?;

    let password_hash = sha3::Sha3_256::digest(user.password.as_bytes());
    let password_hash = format!("{:x}", password_hash);
    user.password=password_hash.clone();

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    diesel::insert_into(users::table)
        .values(&user).execute(&mut connection)
        .map_err(|_| Error::FailInsertDB)?;

    let body = Json(json!({"success":true}));
    Ok(body)
}
pub async fn who_am_i(ctx:Ctx,Extension(pool): Extension<DbPool>)->Result<Json<UserResponse>,Error>{
    let mut connection = pool.get().map_err(|_| Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(ctx.email.clone()))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;

    let response=UserResponse{
        name:user.name,
        surname:user.surname,
        email:user.email
    };
    Ok(Json(response))
}

