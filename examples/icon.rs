use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

pub fn app() -> Element {
    rsx!(
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            style: "font-size: 20px;",
            p { "There is {ICONS_COLLECTION.iter().len()} icons in this collection" }
            for name in ICONS_COLLECTION.iter() {
                match spawn_icon(name) {
                    Some(icon) => rsx!{DxcIcon {
                        children:icon,
                    }},
                    None => rsx!("error loading icon: {name}"),
                }
            }
        }
    )
}
