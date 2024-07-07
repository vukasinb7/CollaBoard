use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use axum::Extension;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use tokio::sync::broadcast;
use crate::{DbPool, Error, RoomState, WSState};
use crate::ctx::Ctx;
use crate::dto::{BoardElement, UpdateBoardPayload};
use crate::model::{Board, Permission, User};
use crate::schema::{boards, permissions, users};
use crate::utils::jwt::decode_jwt;


#[derive(Debug, Serialize, Deserialize)]
struct DrawingPayload {
    id: String,
}

pub async fn handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<WSState>>, Extension(pool): Extension<DbPool>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, pool))
}

async fn handle_socket(socket: WebSocket, state: Arc<WSState>, pool: DbPool) {
    let (mut sender, mut receiver) = socket.split();
    let mut email = String::new();
    let mut board_id = String::new();
    let mut tx = None::<broadcast::Sender<String>>;
    let mut message = "";
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(name) = msg {
            #[derive(Deserialize)]
            struct Connect {
                token: String,
                board_id: String,
            }
            // CONNECT
            let connect: Connect = match serde_json::from_str(&name) {
                Ok(connect) => connect,
                Err(_) => {
                    let _ = sender.send(Message::from("Failed to connect to room!")).await;
                    break;
                }
            };

            {
                use diesel::prelude::*;
                let mut rooms = state.rooms.lock().unwrap();
                board_id = connect.board_id.clone();
                let claim = decode_jwt(connect.token.clone())
                    .map_err(|_| Error::AuthFailCtxNotInRequestExt ).unwrap().claims;
                email = claim.email.clone();
                let mut connection = match pool.get() {
                    Ok(conn) => conn,
                    Err(_) => {
                        message = "Failed to get database connection!";
                        break;
                    }
                };
                let board_id_int = match connect.board_id.parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => {
                        message = "Invalid board ID format!";
                        break;
                    }
                };
                let user = match users::table.filter(users::email.eq(email.clone())).first::<User>(&mut connection) {
                    Ok(user) => user,
                    Err(_) => {
                        message = "User does not exist!";
                        break;
                    }
                };
                let board = match boards::table.filter(boards::id.eq(board_id_int)).first::<Board>(&mut connection) {
                    Ok(board) => {
                        if user.id.ne(&board.owner_id) {
                            match permissions::table
                                .filter(permissions::user_id.eq(&user.id).and(permissions::board_id.eq(&board.id)))
                                .first::<Permission>(&mut connection) {
                                Err(_) => {
                                    message = "Board does not exist";
                                    break;
                                }
                                _ => {}
                            }
                        }
                        board
                    }
                    Err(_) => {
                        message = "Board does not exist";
                        break;
                    }
                };
                let room = rooms.entry(connect.board_id.to_string()).or_insert_with(RoomState::new);
                tx = Some(room.tx.clone());

                if !room.users.lock().unwrap().contains(&email.clone()) {
                    room.users.lock().unwrap().insert(email.clone());
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
    if message.ne("") {
        let _ = sender
            .send(Message::Text(String::from(message)))
            .await;
        message = "";
        return;
    }
    let tx = tx.unwrap();
    let mut rx = tx.subscribe();

    let joined = format!("{} joined the chat!", email);
    let _ = tx.send(joined);


    let mut needed_elements: Vec<String> = Vec::new();
    {
        let mut rooms = state.rooms.lock().unwrap();
        if let Some(room) = rooms.get_mut(&board_id) {
            let buff = room.buff.lock().unwrap();
            for (key, value) in buff.iter() {
                needed_elements.push((*value).clone())
            }
        }
    }
    for element in needed_elements {
        let _ = sender.send(Message::Text(format!("system:{}", element))).await;
    }

    let email_cloned = email.clone();
    let board_id_cloned = board_id.clone();
    let state_cloned = state.clone();
    let mut recv_messages = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if !msg.starts_with(&email_cloned) {
                if sender.send(Message::Text(msg.clone())).await.is_err() {
                    break;
                }
            }
            let mut rooms = state_cloned.rooms.lock().unwrap();
            if let Some(room) = rooms.get_mut(&board_id_cloned) {
                if let Some((_, json_str)) = msg.split_once(": ") {
                    if let Ok(drawing) = serde_json::from_str::<DrawingPayload>(json_str) {
                        let mut buff = room.buff.lock().unwrap();
                        buff.insert(drawing.id.clone(), json_str.to_string());
                    }
                }
            }
        }
    });
    let mut send_messages = {
        let tx = tx.clone();
        let name = email.clone();
        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let _ = tx.send(format!("{}: {}", name, text));
            }
        })
    };

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    }

    let left = format!("{} left the chat!", email);
    let _ = tx.send(left);
    let mut update_db = false;

    {
        let mut rooms = state.rooms.lock().unwrap();
        rooms.get_mut(&board_id).unwrap().users.lock().unwrap().remove(&email);
        if rooms.get_mut(&board_id).unwrap().users.lock().unwrap().len() == 0 {
            update_db = true;
        }
    }
    let board_id_cloned = board_id.clone();
    if update_db {
        println!("DB UPDATING");
        use diesel::prelude::*;
        board_id = board_id_cloned.clone();
        let mut connection = pool.get().unwrap();
        let mut rooms = state.rooms.lock().unwrap();

        let board_id_int = board_id_cloned.parse::<i32>().unwrap();
        let board = boards::table.filter(boards::id.eq(board_id_int)).first::<Board>(&mut connection).unwrap();

        let file_content = fs::read_to_string(board.path.clone()).unwrap();
        let json_data: UpdateBoardPayload = serde_json::from_str::<UpdateBoardPayload>(&file_content).expect("Unable to parse JSON");
        let mut new_state: HashMap<String, String> = HashMap::new();
        for element in json_data.elements {
            let parsed_element = serde_json::from_str::<BoardElement>(&element).expect("Unable to parse JSON");
                new_state.insert(parsed_element.id, element);
        }

        if let Some(room) = rooms.get_mut(&board_id) {
            let buff = room.buff.lock().unwrap();

            for (key, value) in buff.iter() {
                let parsed_element = serde_json::from_str::<BoardElement>(&value).expect("Unable to parse JSON");
                new_state.insert((*key).clone(), (*value).clone());
            }
        }
        let mut elements =vec![];
        for value in new_state.values() {
            let parsed_element = serde_json::from_str::<BoardElement>(&value).expect("Unable to parse JSON");
            if !parsed_element.isDeleted {
                elements.push(json!(value));
            }
        }

        let excalidraw_data_json = json!({
            "elements": elements,
            "appState": {
                "viewBackgroundColor": "#ffffff"
            },
    });
        let file_path = board.path.clone();
        let mut file = File::create(file_path).unwrap();

        let excalidraw_data_string = serde_json::to_string_pretty(&excalidraw_data_json).unwrap();
        file.write_all(excalidraw_data_string.as_bytes()).unwrap();
        rooms.remove(&board_id);
    }
}