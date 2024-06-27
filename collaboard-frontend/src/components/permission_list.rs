use wasm_bindgen_futures::spawn_local;
use yew::{Callback, function_component, Html, html, Properties, use_effect_with, use_state};
use yewdux::use_store;

use crate::api::permission_api::{delete_permission, get_board_permissions, PermissionPayload};
use crate::components::permission_list_item::PermissionListItem;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub(crate) board_id:i32
}

pub struct DeletePermissionPayload{
    pub board_id:i32,
    pub user_email: String,
    pub user_role: i32,
}


#[function_component(PermissionList)]
pub fn board_card(props: &Props) -> Html {
    let board_id=props.board_id;
    let permission_list=use_state(|| Vec::<PermissionPayload>::new());
    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();





    {
        let permission_list = permission_list.clone();
        let token = token.clone();

        use_effect_with((), move |_| {

            spawn_local(async move {
                let resp = get_board_permissions(board_id,&token).await;
                permission_list.set(resp);
            });

            || ()
        });
    }
    html! {
          <div style="display:flex;justify-content:center;flex-direction:column;align-items:center;">
            { for permission_list.iter().map(|permission| html! {
                <PermissionListItem selected_permission={permission.clone()} id={board_id}/>

            }) }
          </div>
  }
}