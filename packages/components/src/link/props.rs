use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};
use dxc_types::types::LinkType;

props! {
    LinkProps {
        #[props_default(value = LinkType::Default)]
        type_: LinkType,

        disabled: bool,

        underline: String,

        href: String,

        target: String,

        class: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
