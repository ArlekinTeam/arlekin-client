use crate::route::{Route, Router};
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlInputElement};
use yew::{function_component, html, Callback, Html};

#[function_component(Login)]
pub fn login() -> Html {
    // Handle submit
    let onsubmit = Callback::from(move |e: yew::events::SubmitEvent| {
        e.prevent_default(); // Prevent from something that is default

        // Access the DOM element
        let email_input = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("username")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        // Access the DOM element v2 (Too lazy to create a function for that)
        let password_input = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("password")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        // Get the values
        let email_value = email_input.value();
        let password_value = password_input.value();

        // Log
        console::log_1(&format!("Email: {}, Password: {}", email_value, password_value).into());
    });

    html! {
        <>
        <Router route={Route::Login}/>
        <form onsubmit={onsubmit}>
            <input
                placeholder="Username"
                name="username"
                id="username"
                type="text"
            />
            <input
                placeholder="Password"
                name="password"
                id="password"
                type="password"
            />
            <button type="submit">{ "Login" }</button>
        </form>
        </>
    }
}
