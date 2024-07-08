use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, Properties, use_state};
use yew_router::hooks::use_navigator;
use crate::api::board_api::BoardCardResponse;
use crate::components::organisms::delete_modal::DeleteModal;
use crate::components::organisms::share_modal::ShareModal;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub selected_board: BoardCardResponse,
}

#[function_component(BoardCard)]
pub fn board_card(props: &Props) -> Html {

  let is_open_share = use_state(|| false);
  let is_open_delete = use_state(|| false);
  let history = use_navigator().unwrap();
  let id=props.selected_board.id.clone();
  let role=props.selected_board.role.clone();

  let onclick_div = Callback::from(move |_| {
    history.push(&Route::Board {id });
  });
  let open_modal_share = {
    let show_modal = is_open_share.clone();
    Callback::from(move |e:MouseEvent|{e.stop_propagation(); show_modal.set(true);})
  };
  let close_modal_share = {
    let show_modal = is_open_share.clone();
    Callback::from(move |e:MouseEvent|{e.stop_propagation(); show_modal.set(false);})
  };
  let open_modal_delete = {
    let show_modal = is_open_delete.clone();
    Callback::from(move |e:MouseEvent|{e.stop_propagation(); show_modal.set(true);})
  };
  let close_modal_delete = {
    let show_modal = is_open_delete.clone();
    Callback::from(move |e:MouseEvent|{e.stop_propagation(); show_modal.set(false);})
  };
  let board=props.selected_board.clone();

  html! {
          <div class="board-card" onclick={onclick_div}>
            <div class="board-card-header" style="display:flex;align-items:center;">
              <p class="board-card-title">{board.name}</p>
              <div>
              if role=="Owner"{<button class="icon-button" onclick={open_modal_share}><img src="static/share-icon.png" style="width:20px;"  alt="share"/></button>}
              if role=="Owner"{<button class="icon-button" onclick={open_modal_delete}><img src="static/delete-icon.png" style="width:20px;"  alt="delete"/></button>}
              </div>
            </div>
            <div class="board-card-footer">
              <p class="board-card-role-name">{board.role}</p>
              <div style="display:flex;flex-direction:row;justify-content:center;">
                <p>{"Created By "}</p>
                <p style="margin-left:5px">{board.owner}</p>
              </div>
            </div>
            if *is_open_share{<ShareModal on_close={close_modal_share} board_id={board.id}/>}
            if *is_open_delete{<DeleteModal on_close={close_modal_delete} board_id={board.id}/>}
          </div>
  }
}