use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    TransitionProps {
        name: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
