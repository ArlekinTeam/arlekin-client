use yew::{function_component, html, Component, Html, HtmlResult, Suspense};
use yew_router::{BrowserRouter, Switch};

use crate::{
    account::{dashboard::Dashboard, login::Login, register::Register},
    route::{self, Route},
};

#[allow(dead_code)]
enum Pages {
    Dashboard,
    Login,
    Register,
}

pub struct App {
    render: Pages,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            render: Pages::Register,
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        match self.render {
            Pages::Dashboard => {
                html! {
                    <Dashboard />
                }
            }
            Pages::Login => {
                html! {
                    <Login />
                }
            }
            Pages::Register => {
                html! {
                    <Register />
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
