use std::sync::Arc;
use axum::Extension;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, RunQueryDsl};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use crate::{DbPool, Error, RoomState, WSState};
use crate::model::{Board, Permission, User};
use crate::schema::{boards, permissions, users};


pub async fn handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<WSState>>,Extension(pool): Extension<DbPool>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, pool))
}

async fn handle_socket(socket: WebSocket, state: Arc<WSState>, pool: DbPool) {
    let (mut sender, mut receiver) = socket.split();
    let mut email = String::new();
    let mut board_id = String::new();
    let mut tx = None::<broadcast::Sender<String>>;
    let mut message="";
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(name) = msg {
            #[derive(Deserialize)]
            struct Connect {
                email: String,
                board_id: String,
            }
            // CONNECT
            let connect: Connect = match serde_json::from_str(&name) {
                Ok(connect) => connect,
                Err(err) => {
                    println!("{}", &name);
                    println!("{}", err);
                    let _ = sender.send(Message::from("Failed to connect to room!")).await;
                    break;
                }
            };

            {
                use diesel::prelude::*;
                let mut rooms = state.rooms.lock().unwrap();
                board_id = connect.board_id.clone();
                email = connect.email.clone();
                let mut connection = match pool.get() {
                    Ok(conn) => conn,
                    Err(_) => {message="Failed to get database connection!";break;}
                };
                let board_id_int = match connect.board_id.parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => {message="Invalid board ID format!";break;
                    }
                };
                let user=match users::table.filter(users::email.eq(email.clone())).first::<User>(&mut connection){
                    Ok(user)=>user,
                    Err(_) => {message="User does not exist!";break; }
                };
                let board=match boards::table.filter(boards::id.eq(board_id_int)).first::<Board>(&mut connection) {
                    Ok(board) => {
                        if user.id.ne(&board.owner_id){
                            match permissions::table
                                .filter(permissions::user_id.eq(&user.id).and(permissions::board_id.eq(&board.id)))
                                .first::<Permission>(&mut connection){
                                Err(_)=>{message="Board does not exist";
                                    break;}
                                _ => {}
                            }
                        }
                        board
                    }
                    Err(_) => {
                        message="Board does not exist";
                        break;
                    }
                };
                let room = rooms.entry(connect.board_id.to_string()).or_insert_with(RoomState::new);
                tx = Some(room.tx.clone());

                if !room.users.lock().unwrap().contains(&connect.email) {
                    room.users.lock().unwrap().insert(connect.email.to_owned());
                    email = connect.email.clone();
                }
            }


            if tx.is_some() && !email.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }
    if message.ne(""){
        let _ = sender
            .send(Message::Text(String::from(message)))
            .await;
        message="";
        return;
    }
    let tx = tx.unwrap();
    let mut rx = tx.subscribe();

    let joined = format!("{} joined the chat!", email);
    let _ = tx.send(joined);
    let welcome = format!("Welcome, {}!", email);
    let _ = sender.send(Message::Text(welcome)).await;

    let email_cloned = email.clone();
    let mut recv_messages = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if !msg.starts_with(&email_cloned) {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    });
    // SEND MESSAGE
    let mut send_messages = {
        let tx = tx.clone();
        let name = email.clone();
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let _ = tx.send(format!("{}: {}", name, text));
                //let mut rooms = state.rooms.lock().unwrap();
                //let mut buff = rooms.get_mut(&board_id).unwrap().buff.lock().unwrap();
                //buff.push_str(&*text);
            }
        })
    };

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    }

    let left = format!("{} left the chat!", email);
    let _ = tx.send(left);
    let mut rooms = state.rooms.lock().unwrap();
    rooms.get_mut(&board_id).unwrap().users.lock().unwrap().remove(&email);

    if rooms.get_mut(&board_id).unwrap().users.lock().unwrap().len() == 0 {
        rooms.remove(&board_id);
    }
}