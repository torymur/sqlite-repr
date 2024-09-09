//! Main UI page.
#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;

use crate::state::{AppState, Format};
use crate::viewer::Viewer;
use crate::{Field, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum NavMove {
    Left,
    Right,
}

fn move_to(direction: NavMove, nf: usize, np: usize) {
    let selected_page = use_context::<AppState>().selected_page;
    let page = selected_page();
    let parts = page.parts();

    let mut selected_field = use_context::<AppState>().selected_field;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut locked_field = use_context::<AppState>().locked_field;

    let (next_nf, next_np) = match direction {
        NavMove::Left => {
            if nf > 0 {
                (nf - 1, np)
            } else if np > 0 {
                let next_np = np - 1;
                let next_nf = &parts[next_np].fields().len() - 1;
                (next_nf, next_np)
            } else {
                // No left - left.
                (nf, np)
            }
        }
        NavMove::Right => {
            let limit = parts[np].fields().len() - 1;
            if nf < limit {
                (nf + 1, np)
            } else if np < &parts.len() - 1 {
                (0, np + 1)
            } else {
                // No right - left.
                (nf, np)
            }
        }
    };

    let part = &parts[next_np];
    let field = &parts[next_np].fields()[next_nf];

    *locked_field.write() = Some((next_np, next_nf));
    *selected_field.write() = Some(field.clone());
    *selected_part.write() = Some(part.clone());
}

fn try_jump(nf: usize, np: usize) {
    let viewer = use_context::<AppState>().viewer;
    let mut selected_page = use_context::<AppState>().selected_page;
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut locked_field = use_context::<AppState>().locked_field;

    let page = &selected_page();
    let field = &page.parts()[np].fields()[nf];
    if let Ok(n) = field.try_page_number() {
        *selected_page.write() = viewer.read().get_page(n);
        *locked_field.write() = None;
        *selected_field.write() = None;
        *selected_part.write() = None;
    }
}

#[component]
pub fn Home(route: Vec<String>) -> Element {
    let locked_field = use_context::<AppState>().locked_field;
    rsx! {
        div {
            class: "focus:outline-none",
            // Allows to have a focus on div, which is necessary to catch keyboard events.
            tabindex: 0,
            onkeydown: move |e| {
                if let Some((np, nf)) = locked_field() {
                    match e.key() {
                        Key::ArrowLeft => move_to(NavMove::Left, nf, np),
                        Key::ArrowRight => move_to(NavMove::Right, nf, np),
                        Key::Enter => try_jump(nf, np),
                        _ => ()
                    }
                }
            },
            Header { }
            Body { }
        }
    }
}

pub fn Header() -> Element {
    let mut current_db = use_context::<AppState>().current_db;
    let mut viewer = use_context::<AppState>().viewer;
    let mut selected_page = use_context::<AppState>().selected_page;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut locked_field = use_context::<AppState>().locked_field;
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
                    class: "join-item select select-secondary select-bordered font-bold tracking-tighter focus:outline-none",
                    oninput: move |e| {
                        *current_db.write() = e.value().to_string();
                        // preloaded databases shouldn't fail
                        let new_viewer = Viewer::new_from_included(e.value().as_str()).expect("Viewer failed");
                        let first_page = new_viewer.get_page(1);
                        *selected_page.write() = first_page;
                        *selected_part.write() = None;
                        *selected_field.write() = None;
                        *locked_field.write() = None;
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
    let mut locked_field = use_context::<AppState>().locked_field;
    rsx! {
        div {
            class: "rounded-box p-4 h-[calc(100vh-48px)] w-fit overflow-y-auto",
            div {
                class: "text-lg font-bold truncate pb-4",
                "🗐  Pages",
            }
            div {
                for (n, page) in pages.into_iter().enumerate() {
                    div {
                        class: "flex w-full",
                        div { class: "flex-grow" }
                        div {
                            class: "leading-tight tracking-tighter font-medium text-cyan-950 text-xs border-r-4 border-cyan-950 pr-1",
                            "{&page.size() * n}", // page offset
                        }
                        button {
                            class: "w-40 h-fit text-left btn-ghost btn-sm btn-block font-medium tracking-tighter truncate",
                            class: if selected_page.read().id() == page.id() {"btn-active"},
                            onclick: move |_| {
                                *selected_page.write() = page.clone();
                                *selected_part.write() = None;
                                *selected_field.write() = None;
                                *locked_field.write() = None;
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
                    FieldNavigation { title: part_label }
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

#[component]
pub fn FieldNavigation(title: String) -> Element {
    let locked_field = use_context::<AppState>().locked_field;
    match locked_field() {
        None => {
            rsx! {
                div {
                    class: "divider items-center",
                    div {
                        class: "text-lg text-center",
                        "{title}"
                    }
                }
            }
        }
        Some((np, nf)) => {
            rsx! {
                div {
                    class: "divider items-center",
                    button {
                        class: "btn btn-sm",
                        onclick: {
                            move |_| move_to(NavMove::Left, nf, np)
                        },
                        "⇦ "
                    }
                    div {
                        class: "text-lg text-center self-center",
                        "{title}"
                    }
                    button {
                        class: "btn btn-sm",
                        onclick: {
                            move |_| move_to(NavMove::Right, nf, np)
                        },
                        "⇨ "
                    }
                }
            }
        }
    }
}

pub fn Visual() -> Element {
    let selected_page = use_context::<AppState>().selected_page;
    let page = selected_page();
    let parts = page.parts();
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
            class: "flex flex-wrap p-4 text-xs",
            for (p, part) in parts.iter().enumerate() {
                for (f, _) in part.fields().iter().enumerate() {
                    FieldElement {nf: f, np: p}
                }
            }
        }
    }
}

#[component]
pub fn FieldElement(nf: usize, np: usize) -> Element {
    let selected_page = use_context::<AppState>().selected_page;
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut trimmed = use_signal(|| true);
    let mut locked = use_context::<AppState>().locked_field;

    let part = &selected_page().parts()[np].clone();
    let field = &part.fields()[nf];
    rsx! {
        div {
            div {
                class: "mb-0 mt-1 pr-2 leading-tight tracking-tighter font-medium text-{part.color()}-800",
                "{field.offset}",
            }
            div {
                class: "p-1 outline outline-1 outline-secondary hover:bg-secondary border-t-4 border-{part.color()}-800 bg-slate-200",
                class: "{field.style}",
                class: if locked() == Some((np, nf)) {"locked"},
                onmouseover: {
                    let part = part.clone();
                    let field = field.clone();
                    move |_| {
                        if locked().is_some() {return}

                        // If field is not locked we want it to move freely.
                        *selected_field.write() = Some(field.clone());
                        *selected_part.write() = Some(part.clone());
                    }
                },
                onclick: {
                    let part = part.clone();
                    let field = field.clone();
                    move |_| {
                        if field.try_page_number().is_ok() {
                            try_jump(nf, np);
                            return;
                        };

                        if let Value::Unallocated(_) = field.value {
                            *trimmed.write() = !trimmed();
                            return;
                        };

                        if locked().is_none() || locked() != Some((np, nf)) {
                            *locked.write() = Some((np, nf));
                            *selected_field.write() = Some(field.clone());
                            *selected_part.write() = Some(part.clone());
                        } else {
                            *locked.write() = None;
                        }
                    }
                },
                FormattedValue {field: field.clone(), trimmed: trimmed()}
            }
        }
    }
}

#[component]
pub fn FormattedValue(field: Rc<Field>, trimmed: bool) -> Element {
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