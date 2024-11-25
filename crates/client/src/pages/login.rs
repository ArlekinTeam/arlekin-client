use crate::helpers::get_by_id::get_by_id;
use web_sys::console;
use yew::{function_component, html, Callback, Html};

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = Callback::from(move |e: yew::events::SubmitEvent| {
        e.prevent_default();

        let email_input = get_by_id("email");

        let password_input = get_by_id("password");

        console::log_1(&format!("Email: {}, Password: {}", email_input, password_input).into());
    });

    html! {
        <>
        <form onsubmit={onsubmit}>
            <input
                placeholder="Email"
                name="email"
                id="email"
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
