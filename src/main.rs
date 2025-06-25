mod components;

use components::panel::Panel;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/compiled.css") }
        header {
            div { class: "title", "Shisō no Senshi" }
        }
        Panel { title: "Test", "Hello world!" }
    }
}
