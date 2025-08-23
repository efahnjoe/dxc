use dioxus::prelude::*;
use super::props::TransitionProps;

#[component]
pub fn DxcTransition(props: TransitionProps) -> Element {
    rsx! {
        {props.children}
    }
}
