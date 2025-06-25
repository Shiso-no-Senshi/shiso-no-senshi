use dioxus::prelude::*;

use super::renderer::{RenderProps, Renderer};

#[derive(Props, PartialEq, Clone)]
pub struct PanelProps<R: Renderer + PartialEq + Clone + 'static> {
    title: String,
    renderer: Option<R>,
    content: String,
}

pub fn Panel<R: Renderer + PartialEq + Clone + 'static>(props: PanelProps<R>) -> Element {
    let mut panel_props: Signal<PanelProps<R>> = use_signal(|| props);

    let render_props: RenderProps = RenderProps {
        content: panel_props.read().content.clone(),
    };

    match &panel_props.read().renderer {
        Some(renderer) => renderer.render(render_props),
        None => {
            rsx! {
                div {}
            }
        }
    }
}
