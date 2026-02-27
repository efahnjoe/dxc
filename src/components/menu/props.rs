use dioxus::prelude::*;
use dxc_macros::{PropsDefault, props};

props! {
    MenuProps {
        class: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
