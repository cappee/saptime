use dioxus::prelude::*;

#[component]
pub fn ThemeToggle(mut dark_mode: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "p-2 rounded-md bg-gray-200 dark:bg-gray-800 text-gray-800 dark:text-gray-200",
            onclick: move |_| {
                dark_mode.set(!dark_mode());
            },
            if dark_mode() {
                "🌙"
            } else {
                "☀️"
            }
        }
    }
}