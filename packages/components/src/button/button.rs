use super::props::ButtonProps;
use crate::DxcIcon;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_icons::spawn_icon;
use dxc_macros::classes;
use dxc_types::namespace::Block;

#[component]
pub fn DxcButton(props: ButtonProps) -> Element {
    let size = use_signal(|| props.size());

    let loading = use_signal(|| props.loading());

    let should_add_space = use_signal(|| false);

    let ns = UseNamespace::new(Block::Button, None);
    let classes = classes!(
        ns.b(),
        ns.m_(props.type_().to_string()),
        ns.m_(size().to_string()),
        ns.is_(String::from("disabled"), Some(props.disabled())),
        ns.is_(String::from("loading"), Some(props.loading())),
        ns.is_(String::from("plain"), Some(props.plain())),
        ns.is_(String::from("round"), Some(props.round())),
        ns.is_(String::from("circle"), Some(props.circle())),
        ns.is_(String::from("text"), Some(props.text())),
        ns.is_(String::from("link"), Some(props.link())),
        ns.is_(String::from("has-bg"), Some(props.bg())),
        props.class(),
    );

    let slot_classes = classes! {
        if should_add_space(){
            &ns.em_(String::from("text"), String::from("expand"))
        }
    };

    rsx! {
        button {
            class: classes,
            style: props.style,
            onclick: move |evt| {
                if let Some(onclick) = props.onclick.as_ref() {
                    onclick.call(evt);
                }
            },
            if loading() {
                DxcIcon {
                    class: ns.is_(String::from("loading"), None),
                    match props.loading_icon {
                        Some(s) => spawn_icon(&s),
                        None => spawn_icon("Loading")
                    }
                }
            } else {
                if props.icon.is_some() {
                    DxcIcon {
                        icon: props.icon().clone(),
                    }
                }
                span {
                    class: slot_classes,
                    {props.children},
                }
            }
        }
    }
}
