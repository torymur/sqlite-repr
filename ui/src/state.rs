//! UI application state

use std::rc::Rc;

use dioxus::prelude::*;

use crate::included_db::SIMPLE_DB;
use crate::viewer::Viewer;
use crate::{Field, PageView, Part};

#[derive(Clone, Debug)]
pub struct AppState {
    pub current_db: Signal<String>,
    pub viewer: Signal<Viewer>,
    pub selected_page: Signal<Rc<dyn PageView>>,
    pub selected_field: Signal<Rc<Field>>,
    pub selected_part: Signal<Rc<dyn Part>>,
    pub locked_field: Signal<Option<(usize, usize)>>,
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
        // preloaded db shouldn't fail
        let viewer =
            Viewer::new_from_included(SIMPLE_DB).expect("Viewer failed to init for preloaded db.");
        let page = viewer.get_page(1);
        let part = viewer.get_part(&page, 0);
        let field = viewer.get_field(&part, 0);

        AppState {
            current_db: Signal::new(SIMPLE_DB.to_string()),
            selected_page: Signal::new(page),
            selected_part: Signal::new(part),
            selected_field: Signal::new(field),
            locked_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
            viewer: Signal::new(viewer),
        }
    }
}
