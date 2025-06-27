#![allow(non_snake_case)]

use dioxus::prelude::*;
use uuid::Uuid;

use super::editor::Editor;

pub struct Panel {
    id: String,
    pub title: String,
    pub editor: Box<dyn Editor>,
}

impl Panel {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn new(title: String, editor: Box<dyn Editor>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            editor,
        }
    }

    pub fn Panel(self: &Self) -> Element {
        rsx! {
            div { id: self.id.clone(), class: "panel", {self.editor.Editor()} }
        }
    }
}
