mod app;
mod logic;
mod api;
mod components;

use app::App;

fn main() {
    dioxus::launch(App);
}
