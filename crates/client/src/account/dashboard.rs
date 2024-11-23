use yew::{function_component, html, Html};

use crate::route::{Route, Router};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <>
        <Router route={Route::Dashboard}/>
        <h1>{ "Dashboard" }</h1>
        </>
    }
}
