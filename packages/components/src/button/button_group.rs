use super::props::ButtonGroupProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::namespace::Block;

#[component]
pub fn DxcButtonGroup(props: ButtonGroupProps) -> Element {
    let ns = UseNamespace::new(Block::Button, None);

    let classes = classes!(ns.b_(String::from("group")), props.class());

    rsx! {
        div {
            class: classes,

            {props.children}
        }
    }
}
