//! UI application state

use std::rc::Rc;

use dioxus::prelude::*;

use crate::ui::header::{Field, Parts};
use crate::ui::viewer::Viewer;

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
        let start_db_name = "Simple";
        // preloaded db shouldn't fail
        let viewer = Viewer::new_from_included(start_db_name)
            .expect("Viewer failed to init for preloaded db.");

        AppState {
            current_db: Signal::new(start_db_name.to_string()),
            selected_part: Signal::new(viewer.first_part()),
            selected_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
            viewer: Signal::new(viewer),
        }
    }
}
