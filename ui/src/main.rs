#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use crate::index::Home;
use crate::state::AppState;

pub mod header;
pub mod index;
pub mod state;
pub mod viewer;

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
