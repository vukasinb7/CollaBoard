use std::fs;
use axum::{Extension, Json};
use axum::extract::{Path, Query};
use chrono::{Duration, Local, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::{Credentials};
use lettre_email::EmailBuilder;
use rand::Rng;
use serde_json::{json, Value};
use crate::{DbPool, Error, utils};
use crate::ctx::Ctx;
use crate::dto::{DeletePermissionParams, InvitationPayload, UserPermission};
use crate::model::{Board, Invitation, NewInvitation, NewPermission, Permission, User};
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
    let html_content = fs::read_to_string("./src/email_template.html")
        .expect("Failed to read email template");

    let rendered_html = html_content.replace("{{board_name}}", &*board.name.clone())
        .replace("{{issuer_name}}", &*user.email.clone())
        .replace("{{board_role}}", match new_invitation.role {2=>"Owner",1=>"Editor",_=>"Viewer"  })
        .replace("{{invitation_link}}", &*String::from(format!("http://127.0.0.1:8080/invitation/{}", new_invitation.code.clone())));


    let email = Message::builder()
        .from("vukasin.bogdanovic610@gmail.com".parse().unwrap())
        .to(invited_user.email.clone().parse().unwrap())
        .subject("Board Invitation")
        .header(ContentType::TEXT_HTML)
        .body(rendered_html)
        .unwrap();

    // Create the SMTP transport
    let creds = Credentials::new((*utils::constants::SMTP_USERNAME).clone(), (*utils::constants::SMTP_PASSWORD).clone());

    let mailer = SmtpTransport::relay("smtp.gmail.com").unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email.clone()).expect("ERR");



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
        return Err(Error::InvitationNotFound);
    }
    if invitation.user_id.ne(&user.id) {
        return Err(Error::InvitationNotFound);
    }
    let current_date_time = Utc::now().naive_utc();
    if invitation.expire<current_date_time{
        return Err(Error::InvitationExpired);
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

    let _ = diesel::delete(invitations::table.filter(invitations::code.eq(invitation.code)))
                               .execute(&mut connection)
                               .map_err(|_| Error::FailDeleteDB);

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


pub async fn get_permission(ctx: Ctx, Extension(pool): Extension<DbPool>, Path(board_id): Path<i32>) -> Result<Json<Value>, Error> {
    use diesel::prelude::*;

    let mut connection = pool.get().map_err(|_|Error::FailToGetPool)?;
    let user = users::table.filter(users::email.eq(&ctx.email))
        .first::<User>(&mut connection)
        .map_err(|_|Error::UserNotFound)?;

    let board = boards::table.filter(boards::id.eq(board_id))
        .first::<Board>(&mut connection)
        .map_err(|_|Error::BoardNotFound)?;

    if board.owner_id.ne(&user.id) {
        permissions::table
            .filter(permissions::user_id.eq(&user.id).and(permissions::board_id.eq(&board.id)))
            .first::<Permission>(&mut connection)
            .map_err(|_| Error::PermissionDenied)?;
    }

    let results = users::table
        .inner_join(permissions::table.on(permissions::user_id.eq(users::id)))
        .filter(permissions::board_id.eq(board_id))
        .select((users::email, permissions::role))
        .load::<UserPermission>(&mut connection)
        .map_err(|_| Error::UserNotFound)?;


    let body = Json(json!(results));
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