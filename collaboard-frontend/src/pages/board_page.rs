use futures::StreamExt;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::use_store;
use crate::api::board_api::{get_board, get_my_boards};

use crate::store::Store;


#[derive(Properties, PartialEq)]
pub struct BoardPageProps {
    pub id: i32,
}
#[function_component(BoardPage)]
pub fn board_page(BoardPageProps { id }: &BoardPageProps) -> Html {
    #[wasm_bindgen(module = "/src/react.js")]
    extern "C" {
        #[wasm_bindgen(js_name = "render_excalidraw")]
        fn render_excalidraw(email:String,id:i32,data:String,role:String);
    }
    let (store, dispatch) = use_store::<Store>();
    let email = store.username.clone();
    let token = store.token.clone();
    let id = *id;


    use_effect_with((), move |_| {
        spawn_local(async move {
            let resp = get_board(id,&token).await;
            render_excalidraw(email,id,resp.data,resp.role)
        });

        || ()
    });

    html! {
        <div>
            <div id="root">

        </div>
        </div>
    }
}



