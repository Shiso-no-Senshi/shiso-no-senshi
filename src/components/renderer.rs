use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RenderProps {
    pub content: String,
}

pub trait Renderer {
    fn render(&self, render_props: RenderProps) -> Element;
}

pub struct DivRenderer {}

impl Renderer for DivRenderer {
    fn render(&self, render_props: RenderProps) -> Element {
        rsx! {
            div { "{render_props.content}" }
        }
    }
}
