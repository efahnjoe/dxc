use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    FooterProps {
        class: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
