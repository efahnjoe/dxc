use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[component]
pub fn DxcFooter(children: Element, class: Option<&'static str>) -> Element {
    let ns = UseNamespace::new("footer", None);

    rsx! {
        div {
            class: format!("{} {}",ns.b(), class.unwrap_or_default()),
            { children }
        }
    }
}
