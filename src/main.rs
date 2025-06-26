mod components;

use components::{
    editor::{Editor, PlainTextEditor},
    group::Group,
    panel::Panel,
};
use dioxus::prelude::*;

#[cfg(feature = "desktop")]
use dioxus::desktop::{Config, WindowBuilder};
fn main() {
    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(desktop!(
                Config::new().with_window(
                    WindowBuilder::new()
                        .with_title("Shisō no Senshi")
                        .with_resizable(true)
                        .with_minimizable(true)
                        .with_maximizable(true)
                        .with_closable(true)
                        .with_background_color((0x0, 0x0, 0x0, 0xff)),
                )
            ))
            .launch(App);
    }

    #[cfg(feature = "web")]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    let mut group: Group = Group::new();

    let mut editor: Box<dyn Editor> = Box::new(PlainTextEditor::new());
    editor.set_content("Hello world!".as_bytes().to_owned());

    let panel: Panel = Panel::new(String::from("Hello world"), editor);
    group.add_panel(panel);

    rsx! {
        document::Stylesheet { href: asset!("/assets/compiled.css") }
        header {
            div { class: "logo",
                img { src: asset!("/assets/images/icon.32.png") }
            }
            div { class: "title", "Shisō no Senshi" }
        }
        {group.Group()}
    }
}
