use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <h1>{ "Home" }</h1>
        </>
    }
}
