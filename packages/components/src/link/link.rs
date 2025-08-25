use super::props::LinkProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::namespace::Block;

#[component]
pub fn DxcLink(props: LinkProps) -> Element {
    let link_type = use_signal(|| props.type_().to_string());
    let underline_mode = use_signal(|| props.underline());
    let disabled = use_signal(|| props.disabled());
    let href = use_signal(|| props.href());
    let target = use_signal(|| props.target());

    let ns = UseNamespace::new(Block::Link, None);

    let classes = classes! {
        ns.b(),
        ns.m_(link_type()),
        ns.is_(String::from("disabled"), Some(disabled())),
        ns.is_(String::from("underline"), Some(underline_mode() == String::from("always"))),
        ns.is_(String::from("hover-underline"), Some(underline_mode() == String::from("hover") && !disabled())),
    };

    rsx! {
        a {
            class: classes,
            href: (!disabled()).then_some(href()),
            target: (!disabled()).then_some(target()),
            onclick: move |e| {
                if disabled() {
                    e.prevent_default();
                }
            },
            span {
                {props.children}
            }
        }
    }
}
