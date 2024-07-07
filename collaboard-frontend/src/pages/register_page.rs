use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use validator::{Validate, ValidationErrors};
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew_router::prelude::use_navigator;
use yewdux::use_store;
use crate::api::board_api::get_board;
use crate::api::user_api;
use crate::api::user_api::{login, register, whoami};
use crate::components::form_input::TextInput;
use crate::routes::Route;
use crate::store::{login_reducer, Store};


#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct RegisterSchema {
    name: String,
    surname: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
    confirm_password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.name = value,
            "surname" => data.surname = value,
            "email" => data.email = value,
            "password" => data.password = value,
            "confirm_password" => data.confirm_password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register() -> Html {
    let form = use_state(|| RegisterSchema::default());

    let name_input_ref = NodeRef::default();
    let surname_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();
    let confirm_password_input_ref = NodeRef::default();

    let handle_name_input = get_input_callback("name", form.clone());
    let handle_surname_input = get_input_callback("surname", form.clone());
    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());
    let handle_confirm_password_input = get_input_callback("confirm_password", form.clone());

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let history = use_navigator().unwrap();
    let (store, store_dispatch) = use_store::<Store>();
    let token = store.token.clone();

    let cloned_history=history.clone();
    let cloned_token=token.clone();
    use_effect_with((token),
                    move |_| {
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = whoami(&cloned_token).await;
                            if resp == 200 {
                                cloned_history.replace(&Route::Home);
                            }
                        });
                    });

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "name" => data.name = value,
                "surname" => data.surname = value,
                "email" => data.email = value,
                "password" => data.password = value,
                "confirm_password" => data.confirm_password = value,
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
        web_sys::console::log_1(&"AWIODJAOWDI".into());
        let cloned_form = form.clone();

        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_surname_input_ref = surname_input_ref.clone();
        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        let cloned_confirm_password_input_ref = confirm_password_input_ref.clone();

        let cloned_history = history.clone();
        let cloned_store_dispatch = store_dispatch.clone();

        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();

            let name_input_ref = cloned_name_input_ref.clone();
            let surname_input_ref = cloned_surname_input_ref.clone();
            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();
            let confirm_password_input_ref = cloned_confirm_password_input_ref.clone();

            let history = cloned_history.clone();
            let store_dispatch = cloned_store_dispatch.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();

                        let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                        let surname_input = surname_input_ref.cast::<HtmlInputElement>().unwrap();
                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                        let confirm_password_input = confirm_password_input_ref.cast::<HtmlInputElement>().unwrap();

                        name_input.set_value("");
                        surname_input.set_value("");
                        email_input.set_value("");
                        password_input.set_value("");
                        confirm_password_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let resp = user_api::register(&form_json).await;
                        history.push(&Route::Login);
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };


    html! {
      <div id="login-container">
        <div id="login-modal">
            <div class="column login-form-container">
                <p>{"Sign Up"}</p>
                <img src="static/logo.png" style="width:300px;margin-bottom:30px"  alt="logo image"/>
                <form style="width:100%; display:flex;flex-direction:column;align-items:center" onsubmit={on_submit}>
                    <div style="height:260px;width:100%;display:flex;justify-content:center;align-items:center;flex-direction:column">
                        <TextInput label="Name" name="name" input_type="text" input_ref={name_input_ref} handle_onchange={handle_name_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                        <TextInput label="Surname" name="surname" input_type="text" input_ref={surname_input_ref} handle_onchange={handle_surname_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                        <TextInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                        <TextInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                        <TextInput label="Confirm Password" name="confirm_password" input_type="password" input_ref={confirm_password_input_ref} handle_onchange={handle_confirm_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                    </div>
                    <button style="margin-top:40px;" class="boton-elegante" href="#">{"Sign Up"}</button>
                </form>
            </div>
            <div class="column login-modal-background">
                <img src="static/loginbg.jpg"  alt="rust image"/>
            </div>

        </div>
        </div>
    }
}