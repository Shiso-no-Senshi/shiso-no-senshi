#![allow(non_snake_case)]

use dioxus::prelude::*;

use super::panel::Panel;

pub struct Group {
    panels: Vec<Panel>,
}

impl Group {
    pub fn new() -> Self {
        Self { panels: vec![] }
    }

    pub fn add_panel(self: &mut Self, panel: Panel) {
        self.panels.push(panel);
    }

    pub fn remove_panel(self: &mut Self, panel: Panel) {
        self.panels.remove(
            self.panels
                .iter()
                .position(|p| p.id() == panel.id())
                .unwrap(),
        );
    }

    pub fn Group(&self) -> Element {
        rsx! {
			GroupHeader { tab_titles: self.panels.iter().map(|panel| panel.title.clone()).collect() }
			div { class: "group", {self.panels.iter().map(|panel| panel.Panel())} }
		}
    }
}

#[component]
fn GroupHeader(tab_titles: Vec<String>) -> Element {
    rsx! {
		div { class: "group-header",
			{tab_titles.iter().map(|tab_title| rsx! {
				Tab { title: tab_title }
			})}
		}
	}
}

#[component]
fn Tab(title: String) -> Element {
    rsx! {
		div { class: "tab",
			div { class: "title", "{title}" }
			TabCloseButton {}
		}
	}
}

#[component]
fn TabCloseButton() -> Element {
    rsx! {
		div { class: "tab-close-button", dangerous_inner_html: "&cross;" }
	}
}
