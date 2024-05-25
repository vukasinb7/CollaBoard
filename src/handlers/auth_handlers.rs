use axum::{Extension, Json};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use serde_json::{json, Value};
use sha3::Digest;
use crate::db::establish_connection;
use crate::{DbPool, Error};
use crate::model::{LoginPayload, NewUser, User};
use crate::schema::users;
use crate::utils::jwt::encode_jwt;

pub async fn login(Extension(pool): Extension<DbPool>, payload: Json<LoginPayload>) -> Result<Json<Value>, Error> {
    use crate::schema::users::dsl::*;
    let mut connection = pool.get().expect("Error getting pool");
    let user = users.filter(email.eq(payload.email.clone())).first::<User>(&mut connection).expect("User not found");
    let password_hash = sha3::Sha3_256::digest(
        payload.password.as_bytes()
    );
    let password_hash = format!("{:x}", password_hash);
    if user.password.eq(&password_hash.clone()) {
        let token = encode_jwt(payload.email.clone())
            .map_err(|_| Error::LoginFail)?;

        let body = Json(json!({"token":token}));
        Ok(body)
    } else {
        Err(Error::LoginFail)
    }
}

pub async fn register(Extension(pool): Extension<DbPool>, Json(mut user): Json<NewUser>) -> Result<Json<Value>, Error> {
    use crate::schema::users::dsl::*;
    let password_hash = sha3::Sha3_256::digest(
        user.password.as_bytes()
    );
    let password_hash = format!("{:x}", password_hash);
    user.password=password_hash.clone();
    let mut connection = pool.get().expect("Error getting pool");
    diesel::insert_into(users)
        .values(&user).execute(&mut connection).expect("Error while inserting user");

    let body = Json(json!({"success":true
    }));
    Ok(body)
}

