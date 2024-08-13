#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use crate::ui::index::Home;
use crate::ui::state::AppState;

pub mod parser;
pub mod ui;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("Failed to init logger.");
    launch(App);
}

fn App() -> Element {
    use_context_provider(AppState::init);
    rsx! {
        Router::<Route> {}
    }
}
