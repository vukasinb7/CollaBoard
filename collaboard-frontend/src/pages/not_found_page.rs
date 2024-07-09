use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::use_store;

use crate::api::user_api::whoami;
use crate::routes::Route;
use crate::store::Store;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();
    let history = use_navigator().unwrap();

    let cloned_history = history.clone();
    let cloned_token = token.clone();
    use_effect_with(token, move |_| {
        spawn_local(async move {
            let resp = whoami(&cloned_token).await;
            if resp == 200 {
                cloned_history.replace(&Route::Home);
            }
            else{
                cloned_history.replace(&Route::Login);
            }
        });
    });

    html! {
        <div/>
    }
}



