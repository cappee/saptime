mod app;
mod logic;
mod api;
mod components;

use app::App;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}
