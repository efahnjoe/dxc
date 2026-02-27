use super::props::MainProps;
use crate::hooks::UseNamespace;
use crate::types::namespace::Block;
use dioxus::prelude::*;
use dxc_macros::classes;

#[component]
pub fn DxcMain(props: MainProps) -> Element {
    let ns = UseNamespace::new(Block::Main, None);

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
