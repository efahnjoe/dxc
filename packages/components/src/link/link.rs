use dioxus:: prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::{classes,props};

props! {
    LinkProps {
        type_: String,
        disabled: bool,
        underline: String,
        href: String,
        target: String,
        children: Element,
    }
}

#[component]
pub fn DxcLink(props: LinkProps) -> Element {
    let link_type = props.type_.as_deref().unwrap_or("default");
    let underline_mode = props.underline.as_deref().unwrap_or("hover");
    let disabled = props.disabled.unwrap_or(false);

    let ns = UseNamespace::new("link", None);

    let classes = classes!{
        ns.b(),
        ns.m_(link_type),
        ns.is_("disabled", Some(disabled)),
        ns.is_("underline", Some(underline_mode == "always")),
        ns.is_("hover-underline", Some(underline_mode == "hover" && !disabled)),
    };

    rsx! {
        a {
            class: classes,
            href: (!disabled).then_some(props.href.as_deref()).flatten(),
            target: (!disabled).then_some(props.target.as_deref()).flatten(),
            onclick: move |e| {
                if disabled {
                    e.prevent_default();
                }
            },
            span {
                {props.children}
            }
        }
    }
}
