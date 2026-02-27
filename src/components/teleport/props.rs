use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    TeleportProps {
        to: String,
        disabled: bool,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
