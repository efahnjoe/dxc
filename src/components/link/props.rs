use crate::types::components::link::{Type, Underline};
use dioxus::prelude::*;
use dxc_macros::{PropsDefault, props};

props! {
    LinkProps {
        #[props_default(value = Type::Default)]
        type_: Type,

        disabled: bool,

        #[props_default(value = Underline::Hover)]
        underline: Underline,

        href: String,

        target: String,

        class: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
