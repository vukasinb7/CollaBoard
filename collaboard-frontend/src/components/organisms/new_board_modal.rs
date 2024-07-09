use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::components::atoms::toast_notification::toast_notification;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::use_store;

use crate::api::board_api::add_board;
use crate::components::atoms::form_input::TextInput;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_close: Callback<()>,
}
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct AddBoardSchema {
    #[validate(
        length(min = 3, message = "Name must be at least 3 characters")
    )]
    name: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<AddBoardSchema>,
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

#[function_component(NewBoardModal)]
pub fn new_board_modal(props: &Props) -> Html {
    let close_modal = props.on_close.clone();
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();
    let form = use_state(|| AddBoardSchema::default());

    let name_input_ref = NodeRef::default();
    let handle_name_input = get_input_callback("name", form.clone());

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
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
                                .insert(field_name, error.clone());
                        }
                    }
                }
            }
        })
    };

    let on_submit = {
        let form = form.clone();
        let name_input_ref = name_input_ref.clone();
        let token=token.clone();
        let close_modal=close_modal.clone();
        let validation_errors = validation_errors.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let form = form.clone();
            let name_input_ref = name_input_ref.clone();
            let token=token.clone();
            let close_modal=close_modal.clone();
            let validation_errors = validation_errors.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                        name_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let _ = add_board(&form_json,&token).await;
                        close_modal.emit(());
                        toast_notification("Info".to_string(),format!("Created board {}",form_data.name))
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
      <div id="myModal" class="modal">
          <div class="modal-content">
            <div style="display:flex;justify-content:space-between;flex-direction:row;width:100%">
                <div style="width:20px"></div>
                <p style="font-size:25px;font-weight:600;margin:0">{"Add new Board"}</p>
                <button class="icon-button" onclick={move |_| close_modal.emit(())}><img src="static/close.png" style="width:20px;"  alt="close"/></button>
            </div>
            <form onsubmit={on_submit} style="width:100%; display:flex;flex-direction:column;align-items:center">
                <div style="height:150px;width:100%;display:flex;justify-content:center;align-items:center;flex-direction:column">
                    <TextInput label="Name" name="name" input_type="text" input_ref={name_input_ref} handle_onchange={handle_name_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                </div>
                <button style="margin-top:10px;" class="boton-elegante" href="#">{"Add Board"}</button>
            </form>
          </div>
      </div>

    }
}