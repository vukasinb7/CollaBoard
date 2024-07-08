mod routes;
mod pages;
mod api;
mod components;
mod store;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use crate::routes::{Route, switch};



#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
