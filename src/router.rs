// use crate::pages::add_task::AddTask;
use crate::pages::edit_task::EditTask;
use crate::pages::one_task::OneTask;
use crate::pages::{create_account::CreateAccount, home::Home, login::Login, confirmsignup::ConfirmSignUp};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create-account")]
    CreateAccount,
    #[at("/confirmsignup")]
    ConfirmSignUp,
    #[at("/signin")]
    Login, 
    #[at("/get/user")]
    GetData, 
    #[at("/tasks/:id")]
    OneTask { id: u32 },
    #[at("/tasks/:id/edit")]
    EditTask { id: u32 },

}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CreateAccount => html! { <CreateAccount /> },
        Route::ConfirmSignUp => html! { <ConfirmSignUp/> },
        Route::Login => html! { <Login /> },
        Route::GetData => html! { <get_data /> },
        Route::OneTask { id } => html! { <OneTask id={id} /> },
        Route::EditTask { id } => html! { <EditTask id={id} />},
    }
}
