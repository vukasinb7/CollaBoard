use yew::{Callback, function_component, Html, html, Properties, use_state};

use crate::api::board_api::BoardResponse;
use crate::components::share_modal::ShareModal;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub selected_board: BoardResponse,
}

#[function_component(BoardCard)]
pub fn board_card(props: &Props) -> Html {

  let is_open = use_state(|| false);
  let open_modal = {
    let show_modal = is_open.clone();
    Callback::from(move |_| show_modal.set(true))
  };
  let close_modal = {
    let show_modal = is_open.clone();
    Callback::from(move |_| show_modal.set(false))
  };
  let board=props.selected_board.clone();
  html! {
          <div class="board-card">
            <div class="board-card-header" style="display:flex;align-items:center;">
              <p class="board-card-title">{board.name}</p>
              <button class="icon-button" onclick={open_modal}><img src="static/three-dots.png" style="width:20px;"  alt="three dots"/></button>
            </div>
            <div class="board-card-footer">
              <p class="board-card-role-name">{"Owner"}</p>
              <div style="display:flex;flex-direction:row;justify-content: center;">
                <p>{"Created By "}</p>
                <p style="margin-left:5px">{"Vukasin"}</p>
              </div>
            </div>
            if *is_open{<ShareModal on_close={close_modal} board_id={board.id}/>}
          </div>
  }
}