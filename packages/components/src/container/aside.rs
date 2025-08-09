use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use std::collections::HashMap;

#[component]
pub fn DxcAside(
    children: Element,
    class: Option<&'static str>,
    width: Option<&'static str>,
    style: Option<&'static str>,
) -> Element {
    let ns = UseNamespace::new("aside", None);

    let style_str = match width {
        Some(w) => {
            let mut vars = HashMap::new();
            vars.insert("width", w);

            ns.css_var_block(vars)
        }
        None => "".to_string(),
    };

    rsx! {
        div {
            class: format!("{} {}",ns.b(), class.unwrap_or_default()),
            style: style_str,
            { children }
        }
    }
}
