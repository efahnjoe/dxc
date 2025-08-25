use super::props::IconProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_icons::spawn_icon;
use dxc_macros::classes;
use dxc_types::namespace::Block;

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
