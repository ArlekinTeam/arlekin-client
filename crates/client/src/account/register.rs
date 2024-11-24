use crate::helpers::get_by_id::get_by_id;
use crate::route::{Route, Router};
use web_sys::console;
use yew::{function_component, html, Callback, Html};

#[function_component(Register)]
pub fn register() -> Html {
    let onsubmit = Callback::from(move |e: yew::events::SubmitEvent| {
        e.prevent_default();

        let email_input = get_by_id("email");
        let username_input = get_by_id("username");
        let password_input = get_by_id("password");

        console::log_1(
            &format!(
                "Email: {}, Password: {}, Username: {}",
                email_input, password_input, username_input
            )
            .into(),
        );
    });

    html! {
        <>
        <Router route={Route::Register}/>
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
            <input
                placeholder="Username"
                name="username"
                id="username"
                type="username"
            />
            <button type="submit">{ "Register" }</button>
        </form>
        </>
    }
}
