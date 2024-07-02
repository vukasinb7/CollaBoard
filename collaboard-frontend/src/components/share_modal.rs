use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationErrors};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::use_store;

use crate::api::board_api::{add_board};
use crate::api::permission_api::invite_user;
use crate::components::form_input::TextInput;
use crate::components::form_select::{BBSelect, SelectOption};
use crate::components::permission_list::PermissionList;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub board_id: i32,
    pub on_close:  Callback<MouseEvent>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct ShareBoardSchema {
    #[validate(
        length(min = 3, message = "Name must be at least 3 characters")
    )]
    name: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<ShareBoardSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.name = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(ShareModal)]
pub fn share_modal(props: &Props) -> Html {
    let close_modal = props.on_close.clone();
    let version=use_state(|| 1);
    let board_id = props.board_id;
    let priority = use_state(|| "Viewer".to_owned());
    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();
    let form = use_state(|| ShareBoardSchema::default());

    let name_input_ref = NodeRef::default();

    let priority_options = vec![
        SelectOption::new("Viewer", "Viewer", true),
        SelectOption::new("Editor", "Editor", false),
        SelectOption::new("Owner", "Owner", false),
    ];
    let priority_onchange = {
        let priority = priority.clone();
        Callback::from(move |new_priority| {
            priority.set(new_priority);
        })
    };

    let handle_name_input = get_input_callback("name", form.clone());

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let history = use_navigator().unwrap();
    let (_store, store_dispatch) = use_store::<Store>();
    let stop_propagation = {
        Callback::from(move |e:MouseEvent|{e.stop_propagation();})
    };
    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "name" => data.name = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_history = history.clone();
        let cloned_store_dispatch = store_dispatch.clone();
        let cloned_token = token.clone();
        let cloned_close_modal = close_modal.clone();
        let cloned_priority_modal = priority.clone();
        let cloned_validation_errors = validation_errors.clone();
        let cloned_version=version.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();

            let name_input_ref = cloned_name_input_ref.clone();

            let history = cloned_history.clone();
            let store_dispatch = cloned_store_dispatch.clone();
            let token = cloned_token.clone();
            let close_modal = cloned_close_modal.clone();
            let priority = cloned_priority_modal.clone();
            let version=cloned_version.clone();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();

                        let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();

                        name_input.set_value("");

                        let p = (*priority).clone();
                        let input = json!({
                            "user_email": form_data.name,
                            "board_id": board_id,
                            "role": match p.as_str() {
                                    "Viewer" => 0,
                                    "Editor" => 1,
                                    "Owner" => 2,
                                    _ => -1,}
                        });
                        let form_json = serde_json::to_string(&input).unwrap();
                        let _resp = invite_user(&form_json,&token).await;
                        version.set((*version)+1);
                    }

                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
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
            <form onsubmit={on_submit} style="width:100%; display:flex;flex-direction:column;align-items:center">
                <div style="height:100px;width:100%;display:flex;justify-content:center;align-items:end;flex-direction:row;">
                    <TextInput label="Name" name="name" input_type="text" input_ref={name_input_ref} handle_onchange={handle_name_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                    <div style="width:15px;"></div>
                    <BBSelect
                          data_test="priority"
                          id="new-priority"
                          label="Priority"
                          options={priority_options}
                          onchange={priority_onchange}
                        />
                </div>
                <button style="margin-top:10px;" class="boton-elegante" href="#">{"Add user"}</button>
                <PermissionList version={version} board_id={board_id}/>
            </form>
          </div>
      </div>

    }
}