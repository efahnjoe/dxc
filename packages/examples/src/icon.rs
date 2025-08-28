use dioxus::prelude::*;
use dxc::prelude::*;

#[component]
pub fn Icon() -> Element {
    rsx!(
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
