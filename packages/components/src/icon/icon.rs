use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[derive(Clone, Props, PartialEq)]
pub struct IconProps {
    #[props(default)]
    children: Option<Element>,
}

/// Use icons from iconfont.
///
/// see https://www.iconfont.cn/
///
/// # Example
/// ```
/// use dxc::prelude::*;
///
/// rsx! {
///   Icon {
///     Apple {}
///   }
/// }
/// ```
#[component]
pub fn DxcIcon(props:IconProps) -> Element {
    let IconProps {
        children,
    } = props;

    let ns = UseNamespace::new("icon", None);

    rsx! {
      i {
        class: ns.b(),
        {children}
      }
    }
}
