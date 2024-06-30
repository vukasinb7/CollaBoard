use std::future::Future;
use std::ptr::write;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use serde_json::json;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use futures::{SinkExt, StreamExt};
use futures::future::LocalFutureObj;
use futures::FutureExt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use yewdux::use_store;
use crate::api::board_api::get_my_boards;
use crate::store::Store;


#[function_component(BoardPage)]
pub fn board_page() -> Html {
    #[wasm_bindgen(module = "/src/react.jsx")]
    extern "C" {
        #[wasm_bindgen(js_name = "render_excalidraw")]
        fn render_excalidraw(email:String);
    }
    let (store, dispatch) = use_store::<Store>();
    let email = store.username.clone();

    // let mut ws = WebSocket::open("ws://localhost:3000/ws").unwrap();
    // let (mut write, mut read) = ws.split();

    // let send_msg = |msg: String| {
    //         match write.send(Message::Text(msg)) {
    //             Ok(_) => {
    //                 web_sys::console::log_1(&"Message sent successfully".into());
    //             },
    //             Err(err) => {
    //                 eprintln!("Failed to send message: {:?}", err);
    //             }
    //         }
    // };


    // use_effect_with((), move |_| {
    //
    //     spawn_local(async move {
    //
    //         spawn_local(async move {
    //             let email = "vukasin.bogdanovic@example.com";
    //             let board_id = "1";
    //
    //             let json_string = json!({
    //                 "email": email,
    //                 "board_id": board_id
    //             }).to_string();
    //             write.send(Message::Text(String::from(json_string))).await.unwrap();
    //         });
    //
    //         spawn_local(async move {
    //             while let Some(msg_result) = read.next().await {
    //                 match msg_result {
    //                     Ok(msg) => {
    //                         // Handle the WebSocket message
    //                         match msg {
    //                             Message::Text(text) => {
    //                                 web_sys::console::log_1(&text.into());
    //                             },
    //                             _ => {}
    //                         }
    //                     },
    //                     Err(err) => {
    //                         // Handle WebSocket error
    //                         eprintln!("WebSocket error: {:?}", err);
    //                         // You might want to break out of the loop or handle the error differently
    //                     },
    //                 }
    //             }
    //             web_sys::console::log_1(&"WebSocket Closed".into());
    //         });
    //     });
    //
    //     || ()
    // });



    use_effect(|| {
        render_excalidraw(email)
    });


    html! {
        <div>
            <div id="root">

        </div>
        </div>
    }
}



