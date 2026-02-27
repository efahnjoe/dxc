use super::props::ContainerProps;
use crate::hooks::UseNamespace;
use crate::types::{components::container::Direction, namespace::Block};
use dioxus::prelude::*;
use dxc_macros::classes;

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
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use dxc::prelude::*;
///
/// fn app() -> Element {
///     rsx! {
///         DxcContainer {
///             direction: Container::Direction::Horizontal,
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
pub fn DxcContainer(props: ContainerProps) -> Element {
    let is_vertical = use_signal(|| match props.direction() {
        Direction::Vertical => true,
        Direction::Horizontal => false,
    });

    let ns = UseNamespace::new(Block::Container, None);

    let classes = classes! {
        ns.b(),
        ns.is_(Direction::Vertical.to_string(), Some(is_vertical())),
        props.class()
    };

    let style = use_signal(|| props.style());

    rsx! {
        section {
            class: classes,
            style: style(),
            {props.children}
        }
    }
}
