use serde::{Deserialize, Serialize};
use web_sys::{Event, MouseEvent};
use yew::{Callback, function_component, Html, html, Properties, use_state};
use yew_router::hooks::use_navigator;

use crate::api::board_api::BoardResponse;
use crate::components::share_modal::ShareModal;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub selected_board: BoardCardResponse,
}

#[derive(Debug, Serialize,Deserialize,Clone,PartialEq)]
pub struct BoardCardResponse {
  pub id:i32,
  pub name:String,
  pub owner:String,
  pub role:String
}

#[function_component(BoardCard)]
pub fn board_card(props: &Props) -> Html {

  let is_open = use_state(|| false);
  let history = use_navigator().unwrap();
  let id=props.selected_board.id.clone();
  let role=props.selected_board.role.clone();

  let onclick_div = Callback::from(move |_| {
    history.push(&Route::Board {id });
  });
  let open_modal = {
    let show_modal = is_open.clone();
    Callback::from(move |e:MouseEvent|{e.stop_propagation(); show_modal.set(true);})
  };
  let close_modal = {
    let show_modal = is_open.clone();
    Callback::from(move |e:MouseEvent|{e.stop_propagation(); show_modal.set(false);})
  };
  let board=props.selected_board.clone();
  html! {
          <div class="board-card" onclick={onclick_div}>
            <div class="board-card-header" style="display:flex;align-items:center;">
              <p class="board-card-title">{board.name}</p>
              if role=="Owner"{<button class="icon-button" onclick={open_modal}><img src="static/three-dots.png" style="width:20px;"  alt="three dots"/></button>
            }</div>
            <div class="board-card-footer">
              <p class="board-card-role-name">{board.role}</p>
              <div style="display:flex;flex-direction:row;justify-content: center;">
                <p>{"Created By "}</p>
                <p style="margin-left:5px">{board.owner}</p>
              </div>
            </div>
            if *is_open{<ShareModal on_close={close_modal} board_id={board.id}/>}
          </div>
  }
}