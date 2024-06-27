use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::use_store;

use crate::api::board_api::{BoardResponse, get_my_boards};
use crate::components::board_card::BoardCard;
use crate::components::new_board_modal::NewBoardModal;
use crate::store::Store;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let is_open = use_state(|| false);
    let board_list = use_state(|| Vec::<BoardResponse>::new());
    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();

    let open_modal = {
      let show_modal = is_open.clone();
      Callback::from(move |_| show_modal.set(true))
    };
    let close_modal = {
        let show_modal = is_open.clone();
        Callback::from(move |_| show_modal.set(false))
    };

    {
        let board_list = board_list.clone();
        let token = token.clone();

        use_effect_with((), move |_| {

            spawn_local(async move {
                let resp = get_my_boards(&token).await;
                board_list.set(resp);
            });

            || ()
        });
    }
    html! {
      <div id="home-container">
        <h1 style="margin:0;">{"Boards"}</h1>
        <div class="home-grid-container">
          { for board_list.iter().map(|board| html! {
                    <BoardCard key={board.id} selected_board={board.clone()}  />
                }) }
          if *is_open{<NewBoardModal on_close={close_modal}/>}
            <button onclick={open_modal} href="#" class="fab">
              <img src="static/plus.png" alt="plus"/>
              {"New Board"}
            </button>
        </div>
      </div>
    }
}