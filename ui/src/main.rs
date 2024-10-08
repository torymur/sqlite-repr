#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use ui::index::Home;
use ui::state::AppState;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/:..route")]
    Home { route: Vec<String> },
}

fn main() {
    #[cfg(debug_assertions)]
    dioxus_logger::init(Level::INFO).expect("Failed to init logger.");

    #[cfg(not(debug_assertions))]
    dioxus_logger::init(Level::ERROR).expect("Failed to init logger.");

    launch(App);
}

fn App() -> Element {
    use_context_provider(AppState::init);
    rsx! {
        Router::<Route> {}
    }
}
