use futures::StreamExt;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::use_store;

use crate::api::board_api::get_board;
use crate::api::permission_api::accept_permission;
use crate::routes::Route;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct AcceptInvitationPageProps {
    pub code: String,
}
#[function_component(AcceptInvitationPage)]
pub fn accept_invitation_page(AcceptInvitationPageProps { code }: &AcceptInvitationPageProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let email = store.username.clone();
    let token = store.token.clone();
    let code = (*code).clone();
    let history=use_navigator().unwrap();


    use_effect_with((), move |_| {
        let history=history.clone();
        spawn_local(async move {
            let resp = accept_permission(code,&token).await;
            history.push(&Route::Home);
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



