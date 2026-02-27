use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    IconProps {
        id: String,
        class: String,
        style: String,
        icon: String,
        onclick: EventHandler<MouseEvent>,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
