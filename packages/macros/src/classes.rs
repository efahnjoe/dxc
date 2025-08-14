/// Combines multiple class names into a single space-separated `String`, skipping empty ones.
///
/// This macro is useful for dynamically constructing CSS class strings in UI frameworks
/// (e.g., web frontends using Yew, Dioxus, or similar), where you want to conditionally
/// include classes or avoid extra whitespace from empty entries.
///
/// ## Behavior
///
/// - Accepts zero or more expressions that can be converted to [`Display`] (like `&str`, `String`, etc.).
/// - Skips any class name that is an empty string after conversion.
/// - Joins the remaining non-empty class names with a single space.
/// - Trailing commas are allowed (Rust-style).
///
/// ## Examples
///
/// ### Empty input
///
/// ```rust
/// use dxc_macros::classes;
///
/// let class = classes!();
/// assert_eq!(class, "");
/// ```
///
/// ### Basic usage
///
/// ```rust
/// use dxc_macros::classes;
///
/// let class = classes!("btn", "btn-primary");
/// assert_eq!(class, "btn btn-primary");
/// ```
///
/// ### With empty or conditional values
///
/// ```rust
/// use dxc_macros::classes;
///
/// let is_active = true;
/// let size: Option<&str> = Some("large");
/// let theme = "";
///
/// let class = classes!(
///     "button",
///     "btn",
///     if is_active { "active" } else { "" },
///     size.unwrap_or(""),
///     theme, // will be skipped
/// );
///
/// assert_eq!(class, "button btn active large");
/// ```
///
/// ### With trailing comma
///
/// ```rust
/// use dxc_macros::classes;
///
/// let class = classes!("x", "y", "z",);
/// assert_eq!(class, "x y z");
/// ```
///
/// ### Using with expressions and variables
///
/// ```rust
/// use dxc_macros::classes;
///
/// let size = "small";
/// let class = classes!("panel", format!("panel-{}", size));
/// assert_eq!(class, "panel panel-small");
/// ```
///
/// ## Usage Notes
///
/// - All inputs are converted to strings via [`ToString`] or [`Display`].
/// - If all inputs are empty, the result is an empty string.
/// - No extra spaces or duplicates are generated — ideal for HTML `class` attributes.
///
/// This macro helps prevent invalid or bloated class attributes by filtering out empties
/// and reducing boilerplate.
#[macro_export]
macro_rules! classes {
    () => {
        String::new()
    };

    ($($class:expr),* $(,)?) => {{
        let mut classes = Vec::new();
        $(
            let s: String = $class.to_string();
            if !s.is_empty() {
                classes.push(s);
            }
        )*
        classes.join(" ")
    }};
}
