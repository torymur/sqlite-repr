#![allow(non_snake_case)]

use std::include_bytes;
use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use sqlite_repr::parser::Reader;
use sqlite_repr::ui::header::{Field, Parts};

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
    pub selected_field: Signal<Option<Field>>,
    pub format: Signal<Format>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Format {
    Hybrid,
    Hex,
    Text,
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
            selected_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
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
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;

    rsx! {
        div {
            class: "h-12 flex items-center bg-primary",
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
                                *selected_part.write() = first_part;
                                *selected_field.write() = None;
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
            class: "flex w-full",
            div {
                class: "bg-secondary",
                SideBar { }
                div { class: "flex-grow" }
            }
            div {
                class: "flex flex-col w-full",
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
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;
    rsx! {
        div {
            class: "rounded-box p-4 h-[calc(100vh-48px)] w-fit",
            div {
                class: "font-bold truncate pb-4",
                "Structure",
            }
            ul {
                for part in parts {
                    li {
                        button {
                            class: "w-full text-left btn-sm btn-ghost btn-block font-normal truncate",
                            class: if *selected_part.read().label() == *part.label() {"btn-active"},
                            onclick: move |_| {
                                *selected_part.write() = part.clone();
                                *selected_field.write() = None;
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
    let selected_field = use_context::<AppState>().selected_field;
    match selected_field() {
        None => {
            rsx! {
                div {
                    class: "p-4 h-64 w-full overflow-auto",
                    "{selected_part().desc()}"
                }
            }
        }
        Some(field) => {
            rsx! {
                div {
                    class: "p-4 h-64 w-full",
                    div {
                        "{selected_part().desc()}"
                    }
                    div {
                        class: "flex pt-6 text-sm space-x-6",
                        div {
                            class: "w-1/2",
                            "{field.desc}"
                        }
                        div {
                            class: "overflow-auto w-1/2",
                            table {
                                class: "table table-sm",
                                tbody {
                                    tr {
                                        td {
                                            "Offset"
                                        }
                                        td {
                                            "{field.offset} byte(s)"
                                        }
                                    }
                                    tr {
                                        td {
                                            "Size"
                                        }
                                        td {
                                            "{field.size} byte(s)"
                                        }
                                    }
                                    tr {
                                        td {
                                            "Value"
                                        }
                                        td {
                                            "{field.value}"
                                        }
                                    }
                                    tr {
                                        td {
                                            "Hex"
                                        }
                                        td {
                                            "{field.to_hex()}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn Visual() -> Element {
    let selected_part = use_context::<AppState>().selected_part;
    let fields = selected_part().fields();
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut formatting = use_context::<AppState>().format;
    rsx! {
        div {
            class: "flex items-center bg-secondary",
            div { class: "flex-grow" }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                class: if formatting() == Format::Hybrid {"btn-active"},
                onclick: move |_| {
                    *formatting.write() = Format::Hybrid
                },
                "Hybrid",
            }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                class: if formatting() == Format::Hex {"btn-active"},
                onclick: move |_| {
                    *formatting.write() = Format::Hex
                },
                "Hex",
            }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                class: if formatting() == Format::Text {"btn-active"},
                onclick: move |_| {
                    *formatting.write() = Format::Text
                },
                "Text",
            }
        }
        div {
            class: "flex flex-wrap join p-4 text-xs",
            for field in fields {
                div {
                    class: "p-1 outline outline-1 outline-secondary bg-primary join-item hover:bg-secondary",
                    onmouseover: move |_| {
                        *selected_field.write() = Some(field.clone());
                    },
                    FormattedValue {field: field.clone()}
                }
            }
        }
    }
}

#[component]
pub fn FormattedValue(field: Field) -> Element {
    let formatting = use_context::<AppState>().format;
    match formatting() {
        Format::Hybrid => {
            rsx! {
                div {
                    class: "divide-y divide-secondary",
                    div {
                        "{field.value}"
                    }
                    div {
                        "{field.to_hex()}"
                    }
                }
            }
        }
        Format::Hex => {
            rsx! {
                div {
                    "{field.to_hex()}"
                }
            }
        }
        Format::Text => {
            rsx! {
                div {
                    "{field.value}"
                }
            }
        }
    }
}
