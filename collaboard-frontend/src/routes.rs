use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::accept_invitation_page::AcceptInvitationPage;
use crate::pages::board_page::BoardPage;
use crate::pages::home_page::{HomePage};
use crate::pages::login_page::{LoginPage};
use crate::pages::not_found_page::NotFoundPage;
use crate::pages::register_page::RegisterPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/board/:id")]
    Board{id:i32},
    #[at("/invitation/:code")]
    AcceptIntivation{code:String},
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Board {id} => html! { <BoardPage id={id}/> },
        Route::AcceptIntivation {code} => html! { <AcceptInvitationPage code={code} /> },
        Route::Login=> html! { <LoginPage /> },
        Route::Register=> html! { <RegisterPage /> },
        Route::NotFound => html! { <NotFoundPage/> },
    }
}