use super::props::AsideProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::namespace::Block;
use std::collections::HashMap;

#[component]
pub fn DxcAside(props: AsideProps) -> Element {
    let ns = UseNamespace::new(Block::Aside, None);

    let classes = classes!(ns.b(), props.class());

    let style = use_signal(|| {
        let mut vars = HashMap::new();
        vars.insert(String::from("width"), props.width());
        ns.css_var_block(vars)
    });

    rsx! {
        div {
            class: classes,
            style: style,
            { props.children }
        }
    }
}
