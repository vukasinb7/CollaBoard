use std::fs;
use std::fs::File;
use axum::{Extension, Json};
use axum::extract::{Path};
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{DbPool, Error};
use crate::ctx::Ctx;
use crate::dto::{BoardPayload, BoardResponse, UpdateBoardPayload};
use crate::model::{Board, NewBoard, Permission, User};
use crate::schema::{boards, permissions, users};

use validator::{Validate};

pub async fn create_board(ctx: Ctx, Extension(pool): Extension<DbPool>, Json(payload): Json<BoardPayload>) -> Result<Json<Board>, Error> {
    use diesel::prelude::*;
    payload.validate().map_err(|_|Error::BadRequest)?;

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

pub async fn get_board(ctx: Ctx, Extension(pool): Extension<DbPool>, Path(path_board_id): Path<i32>) -> Result<Json<BoardResponse>, Error> {
    let mut connection = pool.get().map_err(|_| Error::FailToGetPool).unwrap();
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;

    let board = boards::table.filter(boards::id.eq(&path_board_id))
        .first::<Board>(&mut connection)
        .map_err(|_| Error::BoardNotFound)?;
    let mut permission=2;
    if board.owner_id.ne(&user.id) {
        let perm=permissions::table
            .filter(permissions::user_id.eq(&user.id).and(permissions::board_id.eq(&board.id)))
            .first::<Permission>(&mut connection)
            .map_err(|_| Error::PermissionDenied)?;
        permission=perm.role;
    }

    let file_content = fs::read_to_string(board.path.clone()).unwrap();
    let json_data: Value = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    let json_string = json_data.to_string();

    let response= BoardResponse{
        id:board.id.clone(),
        name: board.name.clone(),
        data: json_string,
        role: match permission {1=>"Editor".to_string(),2=>"Owner".to_string(),
            _ => "Viewer".to_string()
        },
    };

    Ok(Json(response))
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


