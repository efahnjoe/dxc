use super::props::MenuItemProps;
// use crate::DxcToolTip;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::namespace::Block;

#[component]
pub fn DxcMenuItem(props: MenuItemProps) -> Element {
    let mut active = use_signal(|| false);
    let disabled = use_signal(|| props.disabled());

    let onclick = move || {
        if !disabled() {
            // props.onclick.call(());
        }
    };

    let ns_menu_item = UseNamespace::new(Block::MenuItem, None);

    let classes = classes! {
        ns_menu_item.b(),
        ns_menu_item.is_(String::from("active"), Some(active())),
        ns_menu_item.is_(String::from("disabled"), Some(disabled()))
    };

    rsx! {
        li {
            class: classes,
            role: "menuitem",
            tabindex: "-1",
            onclick: move |_| {
                active.set(true);
            },
        }
    }
}
