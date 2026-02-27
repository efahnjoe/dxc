use super::props::TransitionProps;
use dioxus::prelude::*;

#[component]
pub fn DxcTransition(props: TransitionProps) -> Element {
    rsx! {
        {props.children}
    }
}
