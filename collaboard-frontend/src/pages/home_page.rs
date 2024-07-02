use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::use_store;

use crate::api::board_api::get_my_boards;
use crate::components::board_card::{BoardCard, BoardCardResponse};
use crate::components::new_board_modal::NewBoardModal;
use crate::routes::Route;
use crate::store::{logout, Store};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let is_open = use_state(|| false);
    let board_list = use_state(|| Vec::<BoardCardResponse>::new());
    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();
    let history=use_navigator().unwrap();

    let open_modal = {
      let show_modal = is_open.clone();
      Callback::from(move |_| show_modal.set(true))
    };
    let close_modal = {
        let show_modal = is_open.clone();
        Callback::from(move |_| show_modal.set(false))
    };
    let logout_callback = {
        let dispatch = dispatch.clone();
        let cloned_history = history.clone();
        Callback::from(move |_| {
            let dispatch = dispatch.clone();
            let history = cloned_history.clone();
            logout(dispatch);
            history.push(&Route::Login);
        })
    };

    {
        let board_list = board_list.clone();
        let token = token.clone();
        let is_open = is_open.clone();

        use_effect_with((is_open), move |_| {

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
            <button onclick={logout_callback}  href="#" class="fab-logout" >
              <img src="static/logout.png" alt="plus"/>
            </button>
        </div>
      </div>
    }
}