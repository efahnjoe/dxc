use super::props::FooterProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::namespace::Block;

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
