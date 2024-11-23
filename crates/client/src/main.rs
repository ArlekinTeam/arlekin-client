use app::AppInit;

pub mod account;
pub mod app;
pub mod route;

fn main() {
    yew::Renderer::<AppInit>::new().render();
}