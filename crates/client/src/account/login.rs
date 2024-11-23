use yew::{function_component, html, Html};

use crate::route::{Route, Router};

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
        <Router route={Route::Login}/>
        <h1>{ "Login page" }</h1>
        </>
    }
}
