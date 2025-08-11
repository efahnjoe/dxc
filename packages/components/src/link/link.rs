use dioxus:: prelude::*;
use dxc_hooks::UseNamespace;

#[derive(Clone, Props, PartialEq)]
pub struct LinkProps {
    #[props(default)]
    r#type: Option<String>,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    underline: Option<String>,
    #[props(default)]
    href: Option<String>,
    #[props(default)]
    target: Option<String>,
    #[props(default)]
    children: Option<Element>,
}

#[component]
pub fn DxcLink(props: LinkProps) -> Element {
    let LinkProps {
        r#type,
        disabled,
        underline,
        href,
        target,
        children,
    } = props;

    let link_type = r#type.as_deref().unwrap_or("default");
    let underline_mode = underline.as_deref().unwrap_or("hover");

    let ns = UseNamespace::new("link", None);

    let classes = [
        ns.b(),
        ns.m_(link_type),
        ns.is_("disabled", disabled),
        ns.is_("underline", underline_mode == "always"),
        ns.is_("hover-underline", underline_mode == "hover" && !disabled),
    ];

    rsx! {
        a {
            class: classes.join(" "),
            href: (!disabled).then_some(href.as_deref()).flatten(),
            target: (!disabled).then_some(target.as_deref()).flatten(),
            onclick: move |e| {
                if disabled {
                    e.prevent_default();
                }
            },
            span {
                { children }
            }
        }
    }
}
