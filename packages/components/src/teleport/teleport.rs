use super::props::TeleportProps;
use dioxus::prelude::*;

#[component]
pub fn DxcTeleport(props: TeleportProps) -> Element {
    let disabled = use_signal(|| props.disabled());
    // let to_element = use_signal(|| props.to.unwrap_or(String::new()));

    rsx! {
        if disabled() {
            {props.children}
        } else {
            {props.children}
        }
    }
}
