use super::props::HeaderProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::namespace::Block;

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
