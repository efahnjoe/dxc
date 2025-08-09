use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[component]
pub fn DxcMain(children: Element, class: Option<&'static str>) -> Element {
    let ns = UseNamespace::new("main", None);

    rsx! {
        div {
            class: format!("{} {}",ns.b(), class.unwrap_or_default()),
            { children }
        }
    }
}
