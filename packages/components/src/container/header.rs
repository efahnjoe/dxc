use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[component]
pub fn DxcHeader(children: Element, class: Option<&'static str>) -> Element {
    let ns = UseNamespace::new("header", None);

    rsx! {
        div {
            class: format!("{} {}", ns.b(), class.unwrap_or_default()),
            { children }
        }
    }
}
