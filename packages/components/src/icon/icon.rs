use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_icons::spawn_icon;
use dxc_macros::{classes, props};

props! {
    IconProps {
        id: String,
        class: String,
        style: String,
        icon: String,
        onclick: EventHandler<MouseEvent>,
        children: Element,
    }
}

/// Use icons from iconfont.
///
/// see [iconfont](https://www.iconfont.cn/)
///
/// # Example
/// ```
/// use dxc::prelude::*;
///
/// rsx! {
///   DxcIcon {
///     Apple { }
///   }
/// }
/// ```
#[component]
pub fn DxcIcon(props: IconProps) -> Element {
    let ns = UseNamespace::new("icon", None);
    let classes = classes!{ns.b(), props.class.as_deref().unwrap_or("")};

    rsx! {
      i {
        id: props.id.as_deref(),
        class: classes,
        style: props.style,
        onclick: props.onclick.clone().unwrap_or_default(),
        
        match props.icon {
          Some(icon) => {
            spawn_icon(&icon)
          },
          None => {
            props.children
          }
        }
      }
    }
}
