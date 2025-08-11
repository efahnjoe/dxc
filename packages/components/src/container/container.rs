use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[derive(Clone, Props, PartialEq)]
pub struct ContainerProps {
    #[props(default)]
    direction: Option<String>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    children: Option<Element>,
}

/// A generic container component for grouping and styling content.
///
/// This component serves as a structural wrapper to hold and organize child elements.
/// It does not impose layout behavior by itself but can be styled accordingly
/// using the `class` prop. The `direction` attribute is purely semantic or
/// intended for use by styling systems (e.g., CSS classes that respond to it).
///
/// # Props
///
/// - `direction`: A hint for the preferred arrangement of children.
///   Values: `"horizontal"` (default) or `"vertical"`.
/// 
///   When nested with `DxcHeader` or `DxcFooter`, Plese use `vertically`. Otherwise use `horizontally`.
/// 
/// - `class`: Additional CSS classes to apply to the container element.
///   Default: `"container"`.
/// 
/// - `children`: The content to be rendered inside the container.
///
/// # Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dxc::prelude::*;
///
/// fn app() -> Element {
///     rsx! {
///         DxcContainer {
///             direction: "vertical".to_string(),
///             class: "my-container padded bordered".to_string(),
///             {
///                 h1 { "Welcome" }
///                 p { "This content is wrapped in a container." }
///             }
///         }
///     }
/// }
/// ```
///
/// # Usage Notes
///
/// - Use this component to logically or visually group related UI elements.
/// - The `direction` prop may be used by your CSS (e.g., via attribute selectors)
///   to apply different styles, but it does not alter layout by default.
/// - Combine with CSS for full styling control.
#[component]
pub fn DxcContainer(
    props: ContainerProps
) -> Element {
    let ContainerProps { 
        direction, 
        class, 
        children 
    } = props;

    let is_vertical = match direction.as_deref().unwrap_or("horizontal") {
        "vertical" => true,
        "horizontal" => false,
        _ => false,
    };

    let ns = UseNamespace::new("container", None);

    rsx! {
        section {
            class: format!("{} {} {}", ns.b(), ns.is_("vertical", is_vertical), class.unwrap_or_default()),
            {children}
        }
    }
}
