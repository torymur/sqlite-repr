//! UI application state

use std::rc::Rc;

use dioxus::prelude::*;

use crate::ui::header::{Field, Parts};
use crate::ui::viewer::{Viewer, SIMPLE_DB};

#[derive(Clone, Debug)]
pub struct AppState {
    pub current_db: Signal<String>,
    pub viewer: Signal<Viewer>,
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
        // preloaded db shouldn't fail
        let viewer =
            Viewer::new_from_included(SIMPLE_DB).expect("Viewer failed to init for preloaded db.");

        AppState {
            current_db: Signal::new(SIMPLE_DB.to_string()),
            selected_part: Signal::new(viewer.first_part()),
            selected_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
            viewer: Signal::new(viewer),
        }
    }
}
