mod components;

use dioxus::prelude::*;

use components::{
    editor::{Editor, PlainTextEditor},
    group::Group,
    panel::Panel,
};

#[cfg(feature = "desktop")]
use dioxus::desktop::{Config, WindowBuilder};
fn main() {
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

#[component]
fn App() -> Element {
    let mut group: Group = Group::new();

    let mut editor: Box<dyn Editor> = Box::new(PlainTextEditor::new());
    editor.set_content("Hello world!".as_bytes().to_owned());

    let panel: Panel = Panel::new(String::from("Hello world"), editor);
    group.add_panel(panel);

    rsx! {
        //General icons
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: asset!("/assets/images/favicon.ico"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "16x16",
            href: asset!("/assets/images/icon.16.png"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "32x32",
            href: asset!("/assets/images/icon.32.png"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "48x48",
            href: asset!("/assets/images/icon.48.png"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "152x152",
            href: asset!("/assets/images/icon.152.png"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "1024x1024",
            href: asset!("/assets/images/icon.1024.png"),
        }
        document::Link {
            rel: "icon",
            r#type: "image/svg+xml",
            sizes: "16x16",
            href: asset!("/assets/images/logo.svg"),
        }

        // Apple iOS
        document::Link {
            rel: "apple-touch-icon",
            sizes: "152x152",
            href: asset!("/assets/images/icon.152.png"),
        }
        document::Stylesheet { href: asset!("/assets/styles/reset.css") }
        document::Stylesheet { href: asset!("/assets/styles/colors.css") }
        document::Stylesheet { href: asset!("/assets/styles/app.css") }

        header { class: "app-header",
            div { class: "logo",
                img { src: asset!("/assets/images/icon.32.png") }
            }
            div { class: "title", "Shisō no Senshi" }
        }
        {group.Group()}
    }
}
