use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::api::board_api::get_board;
use crate::api::user_api::whoami;
use crate::routes::Route;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct BoardPageProps {
    pub id: i32,
}
#[function_component(BoardPage)]
pub fn board_page(BoardPageProps { id }: &BoardPageProps) -> Html {
    #[wasm_bindgen(module = "/src/excalidraw.js")]
    extern "C" {
        #[wasm_bindgen(js_name = "render_excalidraw")]
        fn render_excalidraw(token:String,id:i32,data:String,role:String);

    }

    let (store, _) = use_store::<Store>();
    let history=use_navigator().unwrap();
    let token = store.token.clone();
    let id = *id;

    let cloned_history=history.clone();
    let cloned_token=token.clone();

    use_effect_with(token.clone(),
    move |_| {
        spawn_local(async move {
            let resp = whoami(&cloned_token).await;
            if resp != 200 {
                cloned_history.replace(&Route::Login);
            }
        });
    });

    use_effect_with((), move |_| {
        spawn_local(async move {
            let resp = get_board(id,&token).await;
            render_excalidraw(token,id,resp.data,resp.role)
        });

        || ()
    });

    html! {
        <div id="excalidraw-root"/>
    }
}



