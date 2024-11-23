use yew::{function_component, html, Component, Html, HtmlResult, Suspense};
use yew_router::{BrowserRouter, Switch};

use crate::{
    account::{dashboard::Dashboard, login::Login},
    route::{self, Route},
};

pub struct App {
    logged_in: bool,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { logged_in: false }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        match self.logged_in {
            true => {
                html! {
                    <Dashboard />
                }
            }
            false => {
                html! {
                    <Login />
                }
            }
        }
    }
}

#[function_component(AppInit)]
pub fn app_init() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
        <Suspense {fallback}>
            <AppLoader />
        </Suspense>
    }
}

#[function_component(AppLoader)]
pub fn app_loader() -> HtmlResult {
    Ok(html! {
        <BrowserRouter>
            <Switch<Route> render={route::switch} />
        </BrowserRouter>
    })
}
