use yew::Renderer;

pub mod app;
pub mod helpers;
pub mod pages;
pub mod route;

fn main() {
    Renderer::<app::App>::new().render();
}
