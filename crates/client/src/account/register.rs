use crate::helpers::get_by_id::get_by_id;
use crate::route::{Route, Router};
use reqwest::Client;
use serde::Serialize;
use sha2::{Digest, Sha256};
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
            let hashed_password = hash_password(&password_input);

            let _ = register_in_backend(&hashed_password, &username_input, &email_input).await;

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

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);

    format!("{:x}", hasher.finalize())
}

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
    password_hash: String,
}

async fn register_in_backend(
    hashed_password: &str,
    username: &str,
    email: &str,
) -> Result<(), reqwest::Error> {
    let user = User {
        username: username.to_string(),
        email: email.to_string(),
        password_hash: hashed_password.to_string(),
    };

    let client = Client::new();

    let ip = "127.0.0.1";

    let response = client
        .put(format!("http://{}/api/v1/auth/user/register", ip))
        .json(&user)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        #[allow(clippy::useless_format)]
        console::log_1(&format!("Register success!").into());
    } else {
        console::log_1(&format!("Register error! Response code: {}", response.status()).into());
    }

    Ok(())
}
