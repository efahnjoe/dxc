use crate::DxcIcon;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_icons::{spawn_icon, Loading};
use dxc_macros::{classes, props};

props! {
    ButtonProps {
        size: String,
        disabled: bool,
        type_: String,
        icon: String,
        navtive_type: String,
        loading: bool,
        loading_icon: String,
        plain: bool,
        text: bool,
        link: bool,
        bg: bool,
        autofocus: bool,
        round: bool,
        circle: bool,
        color: String,
        dark: bool,
        auto_insert_space: bool,
        tag: Vec<String>,
        handle_click: fn(),

        class: String,
        style: String,
        children: Element,
    }
}

#[component]
pub fn DxcButton(props: ButtonProps) -> Element {
    let ns = UseNamespace::new("button", None);
    let classes = classes!(
        ns.b(),
        ns.m_(props.type_.as_deref().unwrap_or("")),
        ns.m_(props.size.as_deref().unwrap_or("")),
        ns.is_("disabled", Some(props.disabled.unwrap_or(false))),
        ns.is_("loading", Some(props.loading.unwrap_or(false))),
        ns.is_("plain", Some(props.plain.unwrap_or(false))),
        ns.is_("round", Some(props.round.unwrap_or(false))),
        ns.is_("circle", Some(props.circle.unwrap_or(false))),
        ns.is_("text", Some(props.text.unwrap_or(false))),
        ns.is_("link", Some(props.link.unwrap_or(false))),
        ns.is_("has-bg", Some(props.bg.unwrap_or(false))),
        props.class.unwrap_or("".to_string()),
    );

    let loading = use_signal(|| props.loading.unwrap_or(false));
    let should_add_space = use_signal(|| false);

    rsx! {
        button {
            class: classes,
            style: props.style,
            onclick: move |_| {
                if let Some(handler) = props.handle_click {
                    handler();
                }
            },
            if loading() {
                DxcIcon {
                    class: ns.is_("loading", None),
                    match props.loading_icon {
                        Some(s) => spawn_icon(&s),
                        None => spawn_icon("Loading")
                    }
                }
            } else {
                match props.icon {
                    Some(s) => spawn_icon(&s),
                    None => rsx!{ "" }.into(),
                }
                span {
                    class: format!("{}:{}",ns.em_("text", "expand"),should_add_space()),
                    {props.children},
                }
            }
        }
    }
}
