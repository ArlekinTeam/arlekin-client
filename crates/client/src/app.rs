use yew::{function_component, html, Html};
use yew_router::BrowserRouter;

use crate::route::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <yew_router::Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
