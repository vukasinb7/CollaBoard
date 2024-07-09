use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationErrors};
use web_sys::{HtmlInputElement};
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::use_store;

use crate::api::permission_api::invite_user;
use crate::components::atoms::form_input::TextInput;
use crate::components::atoms::form_select::{SelectInput, SelectOption};
use crate::components::atoms::toast_notification::toast_notification;
use crate::components::molecules::permission_list::PermissionList;
use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub board_id: i32,
    pub on_close:  Callback<MouseEvent>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct ShareBoardSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<ShareBoardSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(ShareModal)]
pub fn share_modal(props: &Props) -> Html {
    let close_modal = props.on_close.clone();
    let board_id = props.board_id;
    let version=use_state(|| 1);
    let role = use_state(|| "Viewer".to_owned());
    let (store, _) = use_store::<Store>();
    let token = store.token.clone();
    let form = use_state(|| ShareBoardSchema::default());

    let email_input_ref = NodeRef::default();

    let role_options = vec![
        SelectOption::new("Viewer", "Viewer", true),
        SelectOption::new("Editor", "Editor", false),
        SelectOption::new("Owner", "Owner", false),
    ];
    let role_onchange = {
        let role = role.clone();
        Callback::from(move |new_role| {
            role.set(new_role);
        })
    };

    let handle_email_input = get_input_callback("email", form.clone());

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let stop_propagation = {
        Callback::from(move |e:MouseEvent|{e.stop_propagation();})
    };
    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
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
        let email_input_ref = email_input_ref.clone();
        let token = token.clone();
        let role = role.clone();
        let validation_errors = validation_errors.clone();
        let version=version.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let form = form.clone();
            let validation_errors = validation_errors.clone();

            let email_input_ref = email_input_ref.clone();
            let token = token.clone();
            let role = role.clone();
            let version=version.clone();
            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        toast_notification("Info".to_string(), "Sending invitation...".to_string());
                        let form_data = form.deref().clone();
                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();

                        let role_str = (*role).clone();
                        let input = json!({
                            "user_email": form_data.email,
                            "board_id": board_id,
                            "role": match role_str.as_str() {
                                    "Viewer" => 0,
                                    "Editor" => 1,
                                    "Owner" => 2,
                                    _ => -1,}
                        });
                        let form_json = serde_json::to_string(&input).unwrap();
                        let _ = invite_user(&form_json,&token).await;
                        email_input.set_value("");
                        version.set((*version)+1);
                        toast_notification("Info".to_string(), "Invitation sent successfully".to_string());

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
                    <TextInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                    <div style="width:15px;"></div>
                    <SelectInput
                          data_test="role"
                          id="new-role"
                          label="Role"
                          options={role_options}
                          onchange={role_onchange}
                        />
                </div>
                <button style="margin-top:10px;" class="boton-elegante" href="#">{"Add user"}</button>
                <PermissionList version={version} board_id={board_id}/>
            </form>
          </div>
      </div>

    }
}