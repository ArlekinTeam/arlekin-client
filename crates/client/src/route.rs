use yew::{function_component, html, Html, Properties};
use yew_router::{hooks::use_navigator, Routable};

use crate::app::App;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dashboard")]
    Dashboard,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub route: Route,
}

pub fn switch(_: Route) -> Html {
    html! { <App /> }
}

#[function_component(Router)]
pub fn router(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    navigator.push(&props.route);

    html! {}
}
