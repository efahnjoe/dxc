use dioxus::prelude::*;
use dxc::prelude::*;

const COMMON_LAYOUT_SCSS: Asset = asset!("examples/container/src/common-layout.scss");

pub fn main() {
    dioxus::launch(app);
}

pub fn app() -> Element {
    rsx!(
        document::Link{rel: "stylesheet", href: COMMON_LAYOUT_SCSS}
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            for name in ICONS_COLLECTION.iter() {
                match spawn_icon(name) {
                    Some(icon) => rsx!{DxcIcon {
                        children:icon,
                        // size: "1rem",
                        // color: "red",
                    }},
                    None => rsx!("error loading icon: {name}"),
                }
            }
        }
    )
}
