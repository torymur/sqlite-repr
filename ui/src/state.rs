//! UI application state

use std::rc::Rc;

use dioxus::prelude::*;

use crate::viewer::{Viewer, SIMPLE_DB};
use crate::{Field, PageView, Part};

#[derive(Clone, Debug)]
pub struct AppState {
    pub current_db: Signal<String>,
    pub viewer: Signal<Viewer>,
    pub selected_page: Signal<Rc<dyn PageView>>,
    pub selected_field: Signal<Option<Rc<Field>>>,
    pub selected_part: Signal<Option<Rc<dyn Part>>>,
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

        AppState {
            current_db: Signal::new(SIMPLE_DB.to_string()),
            selected_page: Signal::new(viewer.get_page(1)),
            selected_part: Signal::new(None),
            selected_field: Signal::new(None),
            locked_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
            viewer: Signal::new(viewer),
        }
    }
}
