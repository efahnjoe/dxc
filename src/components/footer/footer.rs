use super::props::FooterProps;
use crate::hooks::UseNamespace;
use crate::types::namespace::Block;
use dioxus::prelude::*;
use dxc_macros::classes;

#[component]
pub fn DxcFooter(props: FooterProps) -> Element {
    let ns = UseNamespace::new(Block::Footer, None);

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
