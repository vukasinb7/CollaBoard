

use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use crate::api::user_api::AuthResponse;

#[derive(Clone, Serialize, Deserialize, Store, PartialEq, Debug)]
#[store(storage = "local")]
pub struct Store {
    pub username: String,
    pub token: String,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            username: Default::default(),
            token: Default::default(),
        }
    }
}


pub fn login_reducer(auth_response: AuthResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.username = auth_response.email;
        store.token = auth_response.token;
    });
}

pub fn logout(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(|store| {
        store.username = String::new();
        store.token = String::new();
    });
}

