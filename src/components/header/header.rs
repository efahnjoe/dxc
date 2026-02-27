use super::props::HeaderProps;
use crate::hooks::UseNamespace;
use crate::types::namespace::Block;
use dioxus::prelude::*;
use dxc_macros::classes;

#[component]
pub fn DxcHeader(props: HeaderProps) -> Element {
    let ns = UseNamespace::new(Block::Header, None);

    let classes = classes! {
        ns.b(),
        props.class(),
    };

    rsx! {
        div {
            class: classes,
            { props.children }
        }
    }
}
