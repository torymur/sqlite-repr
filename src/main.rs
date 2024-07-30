#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use manganis::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_examples: HashMap<&'static str, &'static str>,
    pub current_db: Signal<String>,
}

impl AppState {
    pub fn init() -> Self {
        AppState {
            db_examples: HashMap::from([
                ("Simple", manganis::mg!(file("./examples/simple"))),
                ("Big Page", manganis::mg!(file("./examples/big_page"))),
            ]),
            current_db: Signal::new("Simple".to_string()),
        }
    }
}

fn App() -> Element {
    use_context_provider(|| AppState::init());
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Header { }
    }
}

pub fn Header() -> Element {
    let db_examples = use_context::<AppState>().db_examples;
    let mut current_db = use_context::<AppState>().current_db;

    rsx! {
        div {
            class: "flex items-center bg-primary",
            div {
                class: "text-xl font-bold tracking-tighter pl-4",
                "SQLite File Format",
            }
            div { class: "flex-grow" }
            div {
                class: "join",
                div {
                    class: "join-item btn btn-secondary tracking-tighter font-bold",
                    "Example database"
                }
                select {
                    class: "join-item select select-secondary select-bordered font-bold tracking-tighter",
                    oninput: move |e| {
                        match e.value().as_str() {
                            name => *current_db.write() = name.to_string(),
                        };
                    },
                    for (name, _file) in &db_examples {
                        option {
                            selected: if *name == current_db() {"true"},
                            "{name}",
                        }
                    }
                }
            }
            div { class: "flex-grow" }
            div {
                class: "btn btn-ghost tracking-tighter font-bold",
                "Add Yours",
            }
        }
    }
}
