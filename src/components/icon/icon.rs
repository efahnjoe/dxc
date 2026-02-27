use super::props::IconProps;
use crate::hooks::UseNamespace;
use crate::types::namespace::Block;
use dioxus::prelude::*;
use dxc_icons::spawn_icon;
use dxc_macros::classes;

/// Use icons from iconfont.
///
/// see [iconfont](https://www.iconfont.cn/)
///
/// # Example
/// ```rust,ignore
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
    let id = use_signal(|| props.id());

    let ns = UseNamespace::new(Block::Icon, None);

    let classes = classes! {ns.b(), props.class()};

    let style = use_signal(|| props.style());

    rsx! {
      i {
        id: id(),
        class: classes,
        style: style(),
        onclick: move |evt| {
            if let Some(onclick) = props.onclick.as_ref() {
                onclick.call(evt);
            }
        },

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
