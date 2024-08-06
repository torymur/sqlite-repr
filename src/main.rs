#![allow(non_snake_case)]

use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod parser;

use parser::*;

use std::include_bytes;
pub const SIMPLE_DB: &'static [u8] = include_bytes!("../examples/simple");
pub const BIG_PAGE_DB: &'static [u8] = include_bytes!("../examples/big_page");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_examples: HashMap<&'static str, &'static [u8]>,
    pub current_db: Signal<String>,
    pub current_reader: Signal<Reader>,
    pub selected_part: Signal<Rc<dyn Parts>>,
}

impl AppState {
    pub fn init() -> Self {
        let start_db_name = "Simple";
        let start_db_bytes = SIMPLE_DB;
        // preloaded db wouldn't fail
        let reader = Reader::new(start_db_bytes).unwrap();
        // parts will be promised to be > 0
        let first_part = reader.parts[0].clone();
        AppState {
            db_examples: HashMap::from([
                (start_db_name, start_db_bytes),
                ("Big Page", BIG_PAGE_DB),
            ]),
            current_db: Signal::new(start_db_name.to_string()),
            current_reader: Signal::new(reader),
            selected_part: Signal::new(first_part),
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
        Body { }
    }
}

pub fn Header() -> Element {
    let db_examples = use_context::<AppState>().db_examples;
    let mut current_db = use_context::<AppState>().current_db;
    let mut current_reader = use_context::<AppState>().current_reader;
    let mut selected = use_context::<AppState>().selected_part;

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
                            name => {
                                *current_db.write() = name.to_string();
                                // preloaded databases shouldn't fail
                                let db_bytes = db_examples.get(name).unwrap();
                                let reader = Reader::new(db_bytes).expect("Reader failed");
                                let first_part = reader.parts[0].clone();
                                *selected.write() = first_part;
                                *current_reader.write() = reader;
                            }
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

pub fn Body() -> Element {
    rsx! {
        div {
            class: "flex h-screen w-full",
            div {
                class: "bg-secondary",
                SideBar { }
                div { class: "flex-grow" }
            }
            div {
                class: "flex flex-col h-screen w-full",
                div {
                    Description { }
                }
                div {
                    Visual { }
                }
                div { class: "flex-grow" }
            }

        }
    }
}

pub fn SideBar() -> Element {
    let reader = use_context::<AppState>().current_reader;
    let parts = reader.read().parts.clone();
    let mut selected = use_context::<AppState>().selected_part;
    rsx! {
        div {
            class: "rounded-box p-4 min-h-full w-fit",
            div {
                class: "font-bold truncate pb-4",
                "Structure",
            }
            ul {
                for part in parts {
                    li {
                        button {
                            class: "w-full text-left btn-sm btn-ghost btn-block font-normal truncate",
                            class: if *selected.read().label() == *part.label() {"btn-active"},
                            onclick: move |_| {
                                *selected.write() = part.clone();
                            },
                            "✦ {&part.label()}",
                        }
                    }
                }
            }
        }
    }
}

pub fn Description() -> Element {
    let selected_part = use_context::<AppState>().selected_part;
    rsx! {
        div {
            class: "p-4 h-64 w-full overflow-auto",
            "{selected_part().desc()}"
        }
    }
}

pub fn Visual() -> Element {
    let selected_part = use_context::<AppState>().selected_part;
    let raw_bytes = selected_part().bytes();
    let vec_bytes = &raw_bytes.to_vec();
    let text = String::from_utf8_lossy(vec_bytes);
    rsx! {
        div {
            class: "flex items-center bg-secondary",
            div { class: "flex-grow" }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                "Hex",
            }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                "Decimal",
            }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                "Text",
            }
        }
        div {
            class: "text-xs p-4 h-full w-full overflow-auto",
            "{text}",
        }
    }
}
