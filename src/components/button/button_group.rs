use super::props::ButtonGroupProps;
use crate::hooks::UseNamespace;
use crate::types::namespace::Block;
use dioxus::prelude::*;
use dxc_macros::classes;

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
