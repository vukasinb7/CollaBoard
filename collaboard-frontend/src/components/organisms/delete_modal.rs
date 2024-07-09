use yew::prelude::*;
use yewdux::use_store;

use crate::api::board_api::delete_board;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub board_id: i32,
    pub on_close:  Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
pub fn delete_modal(props: &Props) -> Html {
    let close_modal = props.on_close.clone();
    let close_modal_cloned = props.on_close.clone();
    let board_id = props.board_id;
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();

    let stop_propagation = {
        Callback::from(move |e:MouseEvent|{e.stop_propagation();})
    };

    let delete_board_callback = {
        let board_id = board_id.clone();
        let token = token.clone();

        Callback::from( move |e:MouseEvent|{
            e.stop_propagation();
            let board_id = board_id.clone();
            let token = token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let _=delete_board(board_id, &token).await;
                let window = web_sys::window().expect("no global `window` exists");
                window.location().reload().expect("could not reload page");
            });
        })
    };


    html! {
      <div id="myModal" onclick={stop_propagation} class="modal">
          <div class="modal-content">
            <div style="display:flex;justify-content:space-between;flex-direction:row;width:100%">
                <div style="width:20px"></div>
                <p style="font-size:25px;font-weight:600;margin:0">{"Share Board"}</p>
                <button class="icon-button" onclick={move |e:MouseEvent| close_modal.emit(e)}><img src="static/close.png" style="width:20px;"  alt="close"/></button>
            </div>
            <p style="font-size:22px;font-weight:400;margin:0;text-align:center;margin-top:15px">{"Are you sure you want to delete board?"}</p>
            <div style="display:flex;justify-content:space-around;flex-direction:row;width:100%;margin-top:15px">
                <button style="cursor:pointer;color:black;border:1px solid black; border-radius:50px;padding:5px 35px;font-size:20px;font-weight:500;background-color:white;" onclick={move |e:MouseEvent| close_modal_cloned.emit(e)}>{"No"}</button>
                <button style="cursor:pointer;color:white;border:1px solid #3556a7; border-radius:50px;padding:5px 35px;font-size:20px;font-weight:500;background-color:#3556a7;" onclick={delete_board_callback}>{"Yes"}</button>
            </div>
          </div>
      </div>

    }
}