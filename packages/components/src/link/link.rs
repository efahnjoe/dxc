use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

pub struct LinkProps<'a> {
    r#type: &'a str,
    disabled: bool,
    underline: &'a str,
    href: &'a str,
    target: &'a str,
    children: Element,
}

#[component]
pub fn DxcLink(props: LinkProps) -> Element {
    let ns = UseNamespace::new("link", None);

    let m = if props.r#type.is_empty() {
        "default"
    } else {
        props.r#type
    };

    let classes = [
        ns.b(),
        ns.m_(m),
        ns.is_("disabled", props.disabled),
        ns.is_("underline", props.underline == "always"),
        ns.is_(
            "hover-underline",
            props.underline == "hover" && !props.disabled,
        ),
    ];

    rsx! {
        a {
            class: classes.join(" "),
            href: props.href,
            target: props.target,
            onclick: move |e| {
                if props.disabled {
                    e.prevent_default();
                }
            },
            span {
                class: ns.e_("inner"),
                {props.children}
            }
        }
    }
}
