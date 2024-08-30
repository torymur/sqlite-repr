//! Main UI page.

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::state::{AppState, Format};
use crate::viewer::Viewer;

use crate::{Field, Value};

#[component]
pub fn Home(route: Vec<String>) -> Element {
    rsx! {
        Header { }
        Body { }
    }
}

pub fn Header() -> Element {
    let mut current_db = use_context::<AppState>().current_db;
    let mut viewer = use_context::<AppState>().viewer;
    let mut selected_page = use_context::<AppState>().selected_page;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;

    rsx! {
        div {
            class: "h-12 flex items-center bg-slate-200",
            div {
                class: "text-xl font-bold tracking-tighter pl-4",
                "SQLite File Format",
            }
            div {
                class: "pl-4 tooltip tooltip-right",
                "data-tip": "Like the project? Give us a star ☆",
                a {
                    href: "https://github.com/torymur/sqlite-repr",
                    img {
                        class: "h-6 object-scale-down",
                        src: "./github-mark.png"
                    }
                }
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
                        *current_db.write() = e.value().to_string();
                        // preloaded databases shouldn't fail
                        let new_viewer = Viewer::new_from_included(e.value().as_str()).expect("Viewer failed");
                        let first_page = new_viewer.first_page();
                        *selected_page.write() = first_page;
                        *selected_part.write() = None;
                        *selected_field.write() = None;
                        *viewer.write() = new_viewer;
                    },
                    for name in viewer.read().included_dbnames() {
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
    let viewer = use_context::<AppState>().viewer;
    let pages = viewer.read().pages.clone();
    let mut selected_page = use_context::<AppState>().selected_page;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;

    rsx! {
        div {
            class: "rounded-box p-4 h-[calc(100vh-48px)] w-fit",
            div {
                class: "font-bold truncate pb-4",
                "B-tree Pages",
            }
            div {
                for (n, page) in pages.into_iter().enumerate() {
                    div {
                        class: "flex w-full",
                        div { class: "flex-grow" }
                        div {
                            class: "leading-tight tracking-tighter font-medium text-cyan-950 text-xs border-r-4 border-cyan-950 pr-1",
                            // page offset
                            "{&page.page.db_header.page_size * n as u64}",
                        }
                        button {
                            class: "w-40 h-fit text-left btn-ghost btn-sm btn-block font-medium tracking-tighter truncate",
                            class: if selected_page.read().id == page.id {"btn-active"},
                            onclick: move |_| {
                                *selected_page.write() = page.clone();
                                *selected_part.write() = None;
                                *selected_field.write() = None;
                            },
                            "Page {n+1}",
                            br {}
                            "{&page.label()}",
                        }
                    }
                }
            }
        }
    }
}

pub fn Description() -> Element {
    let selected_page = use_context::<AppState>().selected_page;
    let selected_part = use_context::<AppState>().selected_part;
    let selected_field = use_context::<AppState>().selected_field;
    let (part_desc, part_label) = match selected_part() {
        None => ("", "".to_string()),
        Some(p) => (p.desc(), p.label()),
    };
    match selected_field() {
        None => {
            rsx! {
                div {
                    class: "p-4 h-80 w-full overflow-auto",
                    "{selected_page().desc()}"
                }
            }
        }
        Some(field) => {
            rsx! {
                div {
                    class: "p-4 h-80 w-full overflow-auto text-sm",
                    div {
                        "{selected_page().desc()}"
                    }
                    div {
                        class: "text-lg text-center divider",
                        "{part_label}"
                    }
                    div {
                        class: "text-xs",
                        "{part_desc}"
                    }
                    div {
                        class: "flex pt-6 text-xs space-x-6",
                        div {
                            class: "w-2/3",
                            "{field.desc}"
                        }
                        div {
                            class: "w-1/3",
                            table {
                                class: "table table-xs table-fixed",
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
                                            div {
                                                class: "truncate",
                                                "{field.value}"
                                            }
                                        }
                                    }
                                    tr {
                                        td {
                                            "Hex"
                                        }
                                        td {
                                            div {
                                                class: "truncate",
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
}

pub fn Visual() -> Element {
    let selected_page = use_context::<AppState>().selected_page;
    let parts = selected_page().parts();
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut formatting = use_context::<AppState>().format;
    let mut trimmed = use_signal(|| true);
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
            class: "flex flex-wrap p-4 text-xs",
            for part in parts {
                for field in part.fields() {
                    div {
                        div {
                            class: "mb-0 mt-1 pr-2 leading-tight tracking-tighter font-medium text-{part.color()}-800",
                            "{field.offset}",
                        }
                        div {
                            class: "p-1 outline outline-1 outline-secondary bg-slate-200 hover:bg-secondary border-t-4 border-{part.color()}-800",
                            class: "{field.style}",
                            onmouseover:
                            {
                                let part = part.clone();
                                let field = field.clone();
                                move |_| {
                                    *selected_field.write() = Some(field.clone());
                                    *selected_part.write() = Some(part.clone());
                                }
                            },
                            onclick: move |_| {
                                if let Value::Unallocated(_) = field.value { *trimmed.write() = !trimmed()}
                            },
                            FormattedValue {field: field.clone(), trimmed: trimmed()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FormattedValue(field: Field, trimmed: bool) -> Element {
    let formatting = use_context::<AppState>().format;
    let limit: usize = 10;
    let hex = if trimmed {
        field.trim_hex(limit)
    } else {
        field.to_hex()
    };
    let text = if trimmed {
        field.trim_str(limit)
    } else {
        field.value.to_string()
    };
    match formatting() {
        Format::Hybrid => {
            rsx! {
                div {
                    class: "divide-y divide-secondary",
                    div {
                        "{text}"
                    }
                    div {
                        "{hex}"
                    }
                }
            }
        }
        Format::Hex => {
            rsx! {
                div {
                    "{hex}"
                }
            }
        }
        Format::Text => {
            rsx! {
                div {
                    "{text}"
                }
            }
        }
    }
}