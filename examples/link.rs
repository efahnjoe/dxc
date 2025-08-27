use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

#[component]
pub fn app() -> Element {
    rsx!(
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            DxcLink {
                href: "https://example.com".to_string(),
                "Visit Example"
            }
            DxcLink {
                type_: link::Type::Primary,
                disabled: false,
                underline: link::Underline::Always,
                href: Some("https://example.com".to_string()),
                target: Some("_blank".to_string()),
                "Click me"
            }
        }
    )
}
