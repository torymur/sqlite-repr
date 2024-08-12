#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use sqlite_repr::ui::home::Home;
use sqlite_repr::ui::state::AppState;

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
    use_context_provider(|| AppState::init());
    rsx! {
        Router::<Route> {}
    }
}
