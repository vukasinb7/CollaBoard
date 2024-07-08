use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::use_store;

use crate::api::user_api::{login, whoami};
use crate::components::atoms::form_input::TextInput;
use crate::routes::Route;
use crate::store::{login_reducer, Store};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let form = use_state(|| LoginUserSchema::default());

    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let history = use_navigator().unwrap();
    let (store, store_dispatch) = use_store::<Store>();
    let token = store.token.clone();

    let cloned_history=history.clone();
    let cloned_token=token.clone();
    use_effect_with(token,
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
                "email" => data.email = value,
                "password" => data.password = value,
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
        let cloned_form = form.clone();

        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();

        let cloned_history = history.clone();
        let cloned_store_dispatch = store_dispatch.clone();

        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();

            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            let history = cloned_history.clone();
            let store_dispatch = cloned_store_dispatch.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();

                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                        email_input.set_value("");
                        password_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let resp = login(&form_json).await;
                        history.push(&Route::Home);
                        login_reducer(resp, store_dispatch);
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
                <p>{"Sign In"}</p>
                <img src="static/logo.png" style="width:300px;margin-bottom:30px"  alt="logo image"/>
                <form style="width:100%; display:flex;flex-direction:column;align-items:center" onsubmit={on_submit}>
                    <div style="height:150px;width:100%;display:flex;justify-content:center;align-items:center;flex-direction:column">
                        <TextInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                        <TextInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}  />
                    </div>
                    <button style="margin-top:40px;" class="boton-elegante" href="#">{"Sign In"}</button>
                </form>
            </div>
            <div class="column login-modal-background">
                <img src="static/loginbg.jpg"  alt="rust image"/>
            </div>
        </div>
      </div>
    }
}