mod api;
mod app;
mod audio;
mod styles;
mod types;
mod utils;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
