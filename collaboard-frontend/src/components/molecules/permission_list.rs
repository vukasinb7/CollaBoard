use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, Properties, use_effect_with, use_state, UseStateHandle};
use yewdux::use_store;

use crate::api::permission_api::{get_board_permissions, PermissionPayload};
use crate::components::atoms::permission_list_item::PermissionListItem;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub(crate) board_id: i32,
    pub(crate) version: UseStateHandle<i32>,
}

#[function_component(PermissionList)]
pub fn permission_list(props: &Props) -> Html {
    let board_id = props.board_id;
    let permission_list = use_state(|| Vec::<PermissionPayload>::new());
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();

    {
        let permission_list = permission_list.clone();
        let token = token.clone();
        let version = props.version.clone();
        use_effect_with(*version, move |_| {
            spawn_local(async move {
                let resp = get_board_permissions(board_id, &token).await;
                permission_list.set(resp);
            });
            || ()
        });
    }
    html! {
          <div style="display:flex;justify-content:center;flex-direction:column;align-items:center;">
            {for permission_list.iter().map(|permission| html! {
                <PermissionListItem version={props.version.clone()} selected_permission={permission.clone()} id={board_id}/>
            })}
          </div>
  }
}