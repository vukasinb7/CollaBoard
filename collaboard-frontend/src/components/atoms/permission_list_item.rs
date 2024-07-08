use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, Properties, UseStateHandle};
use yewdux::use_store;

use crate::api::permission_api::{delete_permission, PermissionPayload};
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub(crate) selected_permission:PermissionPayload,
    pub(crate) id:i32,
    pub(crate) version:UseStateHandle<i32>
}

#[function_component(PermissionListItem)]
pub fn permission_list_item(props: &Props) -> Html {
    let permission=props.selected_permission.clone();
    let email=permission.email.clone();
    let board_id=props.id;
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();

    let delete_permission_callback = {
        let email=email.clone();
        let board_id = board_id.clone();
        let token = token.clone();
        let version=props.version.clone();
        Callback::from( move |e:MouseEvent|{
            e.stop_propagation();
            let email=email.clone();
            let board_id = board_id.clone();
            let token = token.clone();
            let version=version.clone();
            spawn_local(async move {
                let _=delete_permission(email.clone(), board_id, &token).await;
                version.set((*version)+1);
            });
        })
    };

    html! {
        <div style="display:flex;flex-direction:row;align-items:center;justify-content:space-between;width:100%;margin-top:15px">
            <p>{permission.clone().email}</p>
            <div style="display:flex;flex-direction:row;justify-content:center;margin-left:35px; align-items:center;">
                <p style="margin-right:10px">
                    {match permission.clone().permission_type{
                            1=>"Editor",
                            2=>"Owner",
                            _ => "Viewer",}}</p>
                <button class="icon-button" onclick={delete_permission_callback} >
                    <img src="static/close.png" style="width:20px;"  alt="close"/>
                </button>
            </div>
        </div>
  }
}