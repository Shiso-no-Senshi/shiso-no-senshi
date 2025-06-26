#![allow(non_snake_case)]

use dioxus::prelude::*;

pub trait Editor {
    fn set_content(self: &mut Self, content: Vec<u8>);
    fn get_content(&self) -> Vec<u8>;
    fn Editor(&self) -> Element;
}

pub struct PlainTextEditor {
    content: Vec<u8>,
}

impl PlainTextEditor {
    pub fn new() -> Self {
        Self { content: vec![] }
    }
}

impl Editor for PlainTextEditor {
    fn set_content(self: &mut Self, content: Vec<u8>) {
        self.content = content.clone();
    }

    fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    fn Editor(&self) -> Element {
        rsx! {
            div { class: "plain-text-editor", {String::from_utf8(self.content.clone()).unwrap()} }
        }
    }
}
