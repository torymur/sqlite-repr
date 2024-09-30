//! Main UI page.
#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{
    BsArrowBarLeft, BsArrowBarRight, BsArrowReturnRight, BsArrowRight,
};
use dioxus_free_icons::Icon;

use crate::state::{AppState, Format};
use crate::viewer::Viewer;
use crate::{BTreeNodeView, Field, Value};

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
                class: "pl-4",
                a {
                    href: "https://www.sqlite.org/fileformat2.html",
                    img {
                        class: "h-10 object-scale-down",
                        src: "./sqlite_logo.png"
                    }
                }
            }
            div {
                class: "text-xl font-bold tracking-tighter",
                "SQLite File Format",
            }
            div { class: "flex-grow" }
            div {
                class: "join",
                ExampleDetails { }
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
                class: "flex text-sm items-center tracking-lighter font-thin",
                "Built with",
                a {
                    class: "flex",
                    href: "https://dioxuslabs.com/",
                    img {
                        class: "h-7 object-scale-down",
                        src: "./dioxus_logo.png"
                    }
                }
            }
            div {
                class: "tooltip tooltip-left pl-2 pr-4",
                "data-tip": "Like the project? Give us a star ☆",
                a {
                    href: "https://github.com/torymur/sqlite-repr",
                    img {
                        class: "h-7 object-scale-down",
                        src: "./github-mark.png"
                    }
                }
            }
        }
    }
}

pub fn ExampleDetails() -> Element {
    let current_db = use_context::<AppState>().current_db;
    let viewer = use_context::<AppState>().viewer;
    let rviewer = viewer.read();
    let details = rviewer.included_db.get(current_db().as_str());
    match details {
        None => rsx! { div { } },
        Some((_, desc)) => {
            rsx! {
                div {
                    class: "dropdown dropdown-hover",
                    div {
                        class: "join-item btn bg-secondary border border-secondary tracking-tighter font-bold hover:border-secondary hover:bg-secondary",
                        tabindex: 0,
                        role: "button",
                        "Database Example"
                    }
                    ul {
                        class: "text-xs dropdown-content z-[1] menu bg-secondary shadow w-max tracking-tighter",
                        for line in desc {
                            li {
                                a {
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn Body() -> Element {
    rsx! {
        div {
            class: "flex w-full",
            div {
                class: "flex bg-secondary w-1/5",
                LeftSide { }
            }
            div {
                class: "flex flex-grow flex-col w-4/5 text-xs",
                RightSide { }
            }

        }
    }
}

pub fn RightSide() -> Element {
    rsx! {
        div {
            Description { }
        }
        div {
            Visual { }
        }
    }
}

pub fn LeftSide() -> Element {
    let mut list = use_signal(|| true);
    rsx! {
        div {
            class: "p-4 h-[calc(100vh-48px)] overflow-auto w-full text-sm font-medium",
            div {
                class: "flex w-full",
                div {
                    class: "border border-slate-800 hover:bg-slate-800 hover:text-slate-330",
                    class: if list() {"bg-slate-800 text-slate-330"},
                    onclick: move |_| {
                        list.set(true);
                    },
                    div {
                        class: "p-2",
                        "Page View"
                    }
                }
                div {
                    class: "border border-slate-800 hover:bg-slate-800 hover:text-slate-330",
                    class: if !list() {"bg-slate-800 text-slate-330"},
                    onclick: move |_| {
                        list.set(false);
                    },
                    div {
                        class: "p-2",
                        "Tree View"
                    }
                }
                div { class: "flex-grow border-b border-b-slate-800" }
            }
            div {
                if list() {PageListTab { }} else {PageTreeTab { }}
            }
        }
    }
}

pub fn PageListTab() -> Element {
    let viewer = use_context::<AppState>().viewer;
    let pages = viewer.read().pages.clone();
    let mut selected_page = use_context::<AppState>().selected_page;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut locked_field = use_context::<AppState>().locked_field;
    rsx! {
        div {
            class: "rounded-box p-4 min-w-fit max-w-fit",
            div {
                for (n, page) in pages.into_iter().enumerate() {
                    div {
                        class: "flex",
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

pub fn PageTreeTab() -> Element {
    let viewer = use_context::<AppState>().viewer;
    let btrees = &viewer.read().btrees;
    rsx! {
        div {
            class: "rounded-box min-w-48 max-w-96",
            div {
                class: "join join-vertical w-full",
                for (n, tree) in btrees.iter().enumerate() {
                    div {
                        class: "collapse collapse-arrow join-item border-b border-b-slate-800",
                        input {
                            r#type: "radio",
                            name: "my-accordion-1",
                            "checked": if n == 0 {"true"},
                        }
                        div {
                            class: "collapse-title text-sm capitalize font-medium truncate",
                            div {
                                class: "truncate pb-2",
                                "{tree.name}"
                            }
                            div {
                                class: "text-xs font-normal truncate",
                                "{tree.ttype} Type Btree"
                            }
                            div {
                                class: "text-xs font-normal truncate",
                                "Root Page {tree.root.page_num}"
                            }
                        }
                        div {
                            class: "collapse-content text-xs overflow-x-auto",
                            NodeElement { node: tree.root.clone(), root: true }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn NodeElement(node: BTreeNodeView, root: bool) -> Element {
    let children_interior = node.children.iter().any(|c| c.children.is_empty() == false);
    let node_type = if node.children.is_empty() {
        "Leaf".to_string()
    } else {
        "Interior".to_string()
    };

    let viewer = use_context::<AppState>().viewer;
    let mut selected_page = use_context::<AppState>().selected_page;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut locked_field = use_context::<AppState>().locked_field;
    rsx! {
        div {
            class: "w-full",
            div {
                class: if !root {"px-3"},
                div {
                    div {
                        class: "flex items-center space-x-1 btn-ghost btn-xs btn-block",
                        class: if selected_page.read().id() == node.page_num {"btn-active"},
                        onclick: {

                            let pages = viewer.read().pages.to_vec();
                            move |_| {
                                *selected_page.write() = pages[node.page_num - 1].clone();
                                *selected_part.write() = None;
                                *selected_field.write() = None;
                                *locked_field.write() = None;
                            }
                        },
                        Icon {
                            width: 15,
                            height: 15,
                            icon: BsArrowReturnRight,
                        }
                        div {
                            class: "font-medium",
                            "{node.page_num}"
                        }
                        div {
                            if root {"Root {node_type}"} else {"{node_type}"}
                        }
                    }
                    for page_num in node.overflow {
                        div {
                            class: "flex pl-3 items-center space-x-1 btn-ghost btn-xs btn-block",
                            class: if selected_page.read().id() == page_num {"btn-active"},
                            onclick: {

                                let pages = viewer.read().pages.to_vec();
                                move |_| {
                                    *selected_page.write() = pages[page_num - 1].clone();
                                    *selected_part.write() = None;
                                    *selected_field.write() = None;
                                    *locked_field.write() = None;
                                }
                            },
                            Icon {
                                width: 15,
                                height: 15,
                                icon: BsArrowRight,
                            }
                            div {
                                class: "font-medium",
                                "{page_num}"
                            }
                            div {
                                "Overflow"
                            }
                        }
                    }
                }
                div {
                    class: if children_interior {"flex"} else {"flex-col"},
                    for child in node.children {
                        div {
                            class: "grow",
                            NodeElement {node: child.clone(), root: false}
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
                    class: "p-4 h-80 w-full overflow-auto",
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
                        class: "text-sm font-medium capitalize",
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
                        class: "btn btn-xs btn-ghost focus:outline-none",
                        onclick: {
                            move |_| move_to(NavMove::Left, nf, np)
                        },
                        Icon {
                            icon: BsArrowBarLeft,
                        }
                    }
                    div {
                        class: "text-sm font-medium capitalize",
                        "{title}"
                    }
                    button {
                        class: "btn btn-xs btn-ghost focus:outline-none",
                        onclick: {
                            move |_| move_to(NavMove::Right, nf, np)
                        },
                        Icon {
                            icon: BsArrowBarRight,
                        }
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
