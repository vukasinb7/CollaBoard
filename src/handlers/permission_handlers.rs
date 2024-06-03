use axum::{Extension, Json};
use axum::extract::{Path, Query};
use chrono::{Duration, Local};
use diesel::{ ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;
use serde_json::{json, Value};
use crate::{DbPool, Error};
use crate::ctx::Ctx;
use crate::dto::{DeletePermissionParams, InvitationPayload};
use crate::model::{Board, Invitation, NewInvitation, NewPermission, User};
use crate::schema::{boards, invitations, permissions, users};
use validator::{Validate};

pub async fn create_invitation(ctx: Ctx, Extension(pool): Extension<DbPool>, Json(payload): Json<InvitationPayload>) -> Result<Json<Invitation>, Error> {
    payload.validate().map_err(|_|Error::BadRequest)?;

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;
    let invited_user = users::table.filter(users::email.eq(&payload.user_email))
        .first::<User>(&mut connection)
        .map_err(|_|Error::UserNotFound)?;

    let board = boards::table.filter(boards::id.eq(payload.board_id))
        .first::<Board>(&mut connection)
        .map_err(|_|Error::BoardNotFound)?;

    if user.id.ne(&board.owner_id) {
        Err(Error::PermissionDenied)?
    }

    let invitation = NewInvitation {
        code: generate_random_code(8),
        board_id: payload.board_id,
        user_id: invited_user.id,
        role: payload.role,
        expire: Local::now().naive_local() + Duration::days(2),
    };
    let new_invitation: Invitation = diesel::insert_into(invitations::table)
        .values(&invitation)
        .get_result(&mut connection)
        .map_err(|_|Error::FailInsertDB)?;

    Ok(Json(new_invitation))
}

pub async fn accept_invitation(ctx: Ctx, Extension(pool): Extension<DbPool>, Path(invitation_code): Path<String>) -> Result<Json<Value>, Error> {

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_|Error::UserNotFound)?;
    let invitation: Invitation = invitations::table.filter(invitations::code.eq(&invitation_code))
        .first::<Invitation>(&mut connection)
        .map_err(|_|Error::InvitationNotFound)?;

    if invitation.code.ne(&invitation_code) {
        Err(Error::InvitationNotFound)?
    }
    if invitation.user_id.ne(&user.id) {
        Err(Error::InvitationNotFound)?
    }

    let permission = NewPermission {
        board_id: invitation.board_id,
        user_id: user.id,
        role: invitation.role,
    };
    diesel::insert_into(permissions::table)
        .values(&permission)
        .execute(&mut connection)
        .map_err(|_|Error::FailInsertDB)?;

    let body = Json(json!({"success":true}));
    Ok(body)
}

pub async fn change_permission(ctx: Ctx, Extension(pool): Extension<DbPool>, Json(payload): Json<InvitationPayload>) -> Result<Json<Value>, Error> {
    use diesel::prelude::*;
    payload.validate().map_err(|_|Error::BadRequest)?;

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(ctx.email.clone()))
        .first::<User>(&mut connection).map_err(|_|Error::UserNotFound)?;
    let invited_user = users::table.filter(users::email.eq(payload.user_email.clone()))
        .first::<User>(&mut connection).map_err(|_|Error::FailToGetPool)?;

    let board = boards::table.filter(boards::id.eq(payload.board_id))
        .first::<Board>(&mut connection)
        .map_err(|_|Error::BoardNotFound)?;

    if user.id.ne(&board.owner_id) {
        Err(Error::PermissionDenied)?
    }

    diesel::update(permissions::table
        .filter(permissions::board_id.eq(&board.id).and(permissions::user_id.eq(&invited_user.id))))
        .set(permissions::role.eq(&payload.role))
        .execute(&mut connection)
        .map_err(|_|Error::FailUpdateDB)?;

    let body = Json(json!({"success":true}));
    Ok(body)
}

pub async fn delete_permission(ctx: Ctx, Extension(pool): Extension<DbPool>, Query(payload): Query<DeletePermissionParams>) -> Result<Json<Value>, Error> {
    use diesel::prelude::*;
    payload.validate().map_err(|_|Error::BadRequest)?;

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_|Error::UserNotFound)?;
    let issued_user = users::table.filter(users::email.eq(&payload.user_email))
        .first::<User>(&mut connection)
        .map_err(|_|Error::UserNotFound)?;

    let board = boards::table.filter(boards::id.eq(&payload.board_id))
        .first::<Board>(&mut connection)
        .map_err(|_|Error::BoardNotFound)?;

    if user.id.ne(&board.owner_id) {
        Err(Error::PermissionDenied)?
    }

    diesel::delete(permissions::table.filter(permissions::user_id.eq(&issued_user.id).and(permissions::board_id.eq(&board.id))))
        .execute(&mut connection)
        .map_err(|_|Error::FailDeleteDB)?;

    let body = Json(json!({"success":true}));
    Ok(body)
}


fn generate_random_code(length: usize) -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789";
    let mut rng = rand::thread_rng();

    let code: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    code
}