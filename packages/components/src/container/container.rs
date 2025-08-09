use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[component]
pub fn DxcContainer(
    children: Element,
    class: Option<&'static str>,
    direction: Option<&'static str>,
) -> Element {
    let is_vertical = match direction {
        Some("v") => true,
        Some("h") => false,
        _ => false,
    };

    let ns = UseNamespace::new("container", None);

    rsx! {
        section {
            class: format!("{} {} {}", ns.b(), ns.is_("vertical", is_vertical), class.unwrap_or_default()),
            { children }
        }
    }
}
