use std::fs::File;
use axum::{Extension, Json};
use axum::extract::{Path};
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{DbPool, Error};
use crate::ctx::Ctx;
use crate::dto::{BoardPayload};
use crate::model::{Board, NewBoard, Permission, User};
use crate::schema::{boards, permissions, users};

pub async fn create_board(ctx: Ctx, Extension(pool): Extension<DbPool>, Json(payload): Json<BoardPayload>) -> Result<Json<Board>, Error> {
    use diesel::prelude::*;
    let mut connection = pool.get().map_err(|_| Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(ctx.email.clone()))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;

    let uuid = Uuid::new_v4();
    let board = NewBoard {
        name: payload.name,
        path: format!("./boards/{}.json", uuid),
        owner_id: user.id,
    };
    File::create(board.path.clone()).map_err(|_| Error::FailCreatingFile)?;

    let new_board: Board = diesel::insert_into(boards::table)
        .values(&board)
        .get_result(&mut connection)
        .map_err(|_| Error::FailInsertDB)?;

    Ok(Json(new_board))
}

pub async fn get_board(ctx: Ctx, Extension(pool): Extension<DbPool>, Path(path_board_id): Path<i32>) -> Result<Json<Board>, Error> {
    let mut connection = pool.get().map_err(|_| Error::FailToGetPool).unwrap();
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;

    let board = boards::table.filter(boards::id.eq(&path_board_id))
        .first::<Board>(&mut connection)
        .map_err(|_| Error::BoardNotFound)?;

    if board.owner_id.ne(&user.id) {
        permissions::table
            .filter(permissions::user_id.eq(&user.id).and(permissions::board_id.eq(&board.id)))
            .first::<Permission>(&mut connection)
            .map_err(|_| Error::PermissionDenied)?;
    }
    //TODO: CHECK IF WORKS

    Ok(Json(board))
}

pub async fn get_my_boards(ctx: Ctx, Extension(pool): Extension<DbPool>) -> Result<Json<Vec<Board>>, Error> {
    let mut connection = pool.get().map_err(|_| Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;

    let mut shared_boards = boards::table.inner_join(permissions::table.on(permissions::board_id.eq(&boards::id)))
        .filter(permissions::user_id.eq(&user.id))
        .select(boards::all_columns)
        .load::<Board>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;
    let owner_boards = boards::table.filter(boards::owner_id.eq(&user.id))
        .load::<Board>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;
    shared_boards.extend(owner_boards);

    Ok(Json(shared_boards))
}

pub async fn delete_board(ctx: Ctx, Extension(pool): Extension<DbPool>, Path(path_board_id): Path<i32>) -> Result<Json<Value>, Error> {
    let mut connection = pool.get().map_err(|_| Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;

    let board = boards::table.filter(boards::id.eq(&path_board_id))
        .first::<Board>(&mut connection)
        .map_err(|_| Error::BoardNotFound)?;
    if board.owner_id.ne(&user.id) {
        Err(Error::PermissionDenied)?
    }

    diesel::delete(permissions::table.filter(permissions::board_id.eq(&path_board_id)))
        .execute(&mut connection)
        .map_err(|_|Error::FailDeleteDB)?;

    diesel::delete(boards::table.filter(boards::id.eq(&path_board_id)))
        .execute(&mut connection)
        .map_err(|_|Error::FailDeleteDB)?;

    let body = Json(json!({"success":true}));
    Ok(body)
}

