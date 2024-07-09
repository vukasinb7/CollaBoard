use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::use_store;

use crate::api::permission_api::accept_permission;
use crate::routes::Route;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct AcceptInvitationPageProps {
    pub code: String,
}

#[function_component(AcceptInvitationPage)]
pub fn accept_invitation_page(AcceptInvitationPageProps { code }: &AcceptInvitationPageProps) -> Html {
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();
    let code = (*code).clone();
    let history = use_navigator().unwrap();
    let is_valid_code = use_state(|| true);

    let cloned_is_valid_code = is_valid_code.clone();
    use_effect_with((), move |_| {
        let history = history.clone();
        let is_valid_code = cloned_is_valid_code.clone();
        let code = code.clone();
        let token = token.clone();

        spawn_local(async move {
            let valid = accept_permission(code, &token).await;
            is_valid_code.set(valid);
            TimeoutFuture::new(4000).await;
            history.replace(&Route::Home);
        });

        || ()
    });

    html! {
        <div class="login-container">
            <div class="login-modal" style="display:flex;justify-content:center;align-items:center;flex-direction:column">
            if *is_valid_code {
                <img style="width:350px;height:350px;" src="/static/checked.png" alt="success"/>
                <h1 style="margin:0;margin-top:10px;">{ "Joined Successfully" }</h1>
                <p>{"You will be redirected to boards page in few seconds."}</p>
            } else {
                <img style="width:350px;height:350px;" src="/static/cancel.png" alt="success"/>
                <h1 style="margin:0;margin-top:10px;">{ "Something Went Wrong"}</h1>
                <p>{"You will be redirected to boards page in few seconds."}</p>
            }
        </div>
        </div>
    }
}



