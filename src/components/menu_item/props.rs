use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    MenuItemProps {
        index: usize,

        route: String,

        disabled: bool,
    },
    derive(PropsDefault)
}
