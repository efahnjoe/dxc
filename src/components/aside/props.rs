use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    AsideProps {
        class: String,
        width: String,
        style: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
