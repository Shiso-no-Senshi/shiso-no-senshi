use dioxus::prelude::*;

use super::panel::PanelProps;

#[derive(Props,PartialEq,Clone)]
pub struct GroupProps {
	panels: Vec<PanelProps<>
}

#[component]
pub fn Group() -> Element {
    rsx! {
        div {
            class: "group",
            GroupHeader{ }

        }
    }
}

#[component]
fn GroupHeader(tab_titles: Vec<String>) -> Element {
    rsx! {
        div {
            class: "group-header",
            tab_titles.map(|tab_title| rsx!{Tab { title: tab_title }})
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
