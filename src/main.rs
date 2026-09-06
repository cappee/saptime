mod app;
mod logic;
mod api;
mod components;
mod theme;

use app::App;

fn main() {
    dioxus::launch(App);
}
