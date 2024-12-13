use crate::helpers::get_by_id::get_by_id;
use crate::route::{Route, Router};
use argon2::Argon2;
use hex::encode;
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::{function_component, html, Callback, Html};

#[function_component(Register)]
pub fn register() -> Html {
    let onsubmit = Callback::from(move |e: yew::events::SubmitEvent| {
        e.prevent_default();
        spawn_local(async move {
            let email_input = get_by_id("email");
            let username_input = get_by_id("username");
            let password_input = get_by_id("password");
            let hashed_password = hash_password(&password_input, &email_input);

            match register_in_backend(&hashed_password, &username_input, &email_input).await {
                Ok(()) => {
                    console::log_1(&"Registration successful!".into());
                }
                Err(e) => {
                    console::log_1(&format!("Registration failed: {}", e).into());
                }
            }

            console::log_1(
                &format!(
                    "Email: {}, Password: {}, Username: {}, Hashed Passowrd: {}",
                    email_input, password_input, username_input, hashed_password
                )
                .into(),
            );
        });
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
                type="text"
            />
            <button type="submit">{ "Register" }</button>
        </form>
        </>
    }
}

fn hash_password(password: &str, email: &str) -> String {
    let mut password_hash = [0u8; 32];

    Argon2::default()
        .hash_password_into(
            password.as_bytes(),
            format!("arlekin{}login", email).as_bytes(),
            &mut password_hash,
        )
        .unwrap();

    encode(password_hash)
}

async fn register_in_backend(
    hashed_password: &str,
    username: &str,
    email: &str,
) -> Result<(), Box<dyn Error>> {
    let user = json! ({
        "username": username,
        "email": email,
        "password_hash": hashed_password,
    });

    let client = Client::new();

    let ip = "127.0.0.1";

    let response = client
        .put(format!("http://{}/api/v1/auth/user/register", ip))
        .json(&user)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response.text().await.unwrap();
        Err(format!(
            "Failed to register user status code: {}, error message: {}",
            status_code, error_message
        )
        .into())
    }
}
