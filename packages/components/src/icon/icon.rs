use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

/// Use icons from iconfont.
///
/// see https://www.iconfont.cn/
///
/// # Example
/// ```
/// use create::components::Icon;
///
/// rsx! {
///   Icon {
///     icon: "#icon-name"
///   }
/// }
/// ```
#[component]
pub fn DxcIcon(icon: String) -> Element {
    let ns = UseNamespace::new("icon", None);

    rsx! {
      i {
        class: ns.b(),
      }
    }
}
