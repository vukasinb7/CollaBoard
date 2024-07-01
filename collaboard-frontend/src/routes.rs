use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::board_page::BoardPage;
use crate::pages::home_page::{HomePage};
use crate::pages::login_page::{LoginPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/board/:id")]
    Board{id:i32},
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Board {id} => html! { <BoardPage id={id} /> },
        Route::Login=> html! { <LoginPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        _ => html!{<h1>{"Not Found"}</h1>}
    }
}