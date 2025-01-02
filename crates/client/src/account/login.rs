use crate::helpers::get_by_id::get_by_id;
use crate::route::{Route, Router};
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use wasm_cookies::{cookies, CookieOptions};
use web_sys::console;
use yew::{function_component, html, Callback, Html};

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = Callback::from(move |e: yew::events::SubmitEvent| {
        e.prevent_default();

        wasm_bindgen_futures::spawn_local(async move {
            let email_input = get_by_id("email");

            let password_input = get_by_id("password");

            console::log_1(&format!("Email: {}, Password: {}", email_input, password_input).into());
            match login_in_backend(&email_input, &password_input).await {
                Ok(()) => {
                    console::log_1(&"Login successful!".into());
                }
                Err(e) => {
                    console::log_1(&format!("Login failed: {}", e).into());
                }
            }
        });
    });

    html! {
        <>
        <Router route={Route::Login}/>
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
async fn login_in_backend(email: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let ip = "127.0.0.1";

    let user = json!({
            "email": email,
            "password_hash": password
    });

    let response = client
        .post(format!("http://{}/api/v1/auth/user/login", ip))
        .json(&user)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    if response.status().is_success() {
        console::log_1(&"Login successful".into());
        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let user_id = response_json["user_id"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        console::log_1(&format!("Snowflake: {}", user_id).into());
        cookies::set("user_id", user_id.as_str(), &CookieOptions::default());
    } else {
        console::log_1(&format!("Login failed, response: {}", response.status()).into());
    }
    Ok(())
}
