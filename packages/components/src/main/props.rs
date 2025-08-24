use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    MainProps {
        class: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
