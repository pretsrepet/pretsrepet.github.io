use yew::Renderer;

mod types;
mod app;
mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
