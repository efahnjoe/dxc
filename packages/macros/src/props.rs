/// Creates a new public struct with optional fields and derives common traits.
///
/// This macro generates a `pub` struct where each field is wrapped in `Option<T>`,
/// and automatically derives [`Clone`], [`PartialEq`], and [`Debug`] by default.
/// You can optionally specify additional traits to derive using the `derive:` keyword.
///
/// # Usage
///
/// Basic usage:
/// ```rust
/// use dxc_macros::props;
///
/// props! {
///     MyProps {
///         name: String,
///         age: i32,
///     }
/// }
/// ```
///
/// This expands to:
/// ```rust
/// #[derive(Clone, PartialEq, Debug)]
/// pub struct MyProps {
///     pub name: Option<String>,
///     pub age: Option<i32>,
/// }
/// ```
///
/// To derive additional traits (e.g., `Copy`), include the `derive:` clause:
/// ```rust
/// use dxc_macros::props;
///
/// props! {
///     MyProps {
///         x: f64,
///         y: f64,
///     },
///     derive: [Copy]
/// }
/// ```
///
/// This expands to:
/// ```rust
/// #[derive(Clone, PartialEq, Debug, Copy)]
/// pub struct MyProps {
///     pub x: Option<f64>,
///     pub y: Option<f64>,
/// }
/// ```
///
/// > **Note**: When deriving `Copy`, all field types must also implement `Copy`.
///
/// # Field Semantics
///
/// All generated fields are of type `Option<T>`, representing that they are optional
/// and default to `None`. This is useful for builder-pattern or configuration structs.
///
/// # Supported Derives
///
/// You can append any valid trait to `derive:` as long as it's supported by Rust's `#[derive(...)]`,
/// such as:
/// - `Copy`
/// - `Default`
/// - `PartialOrd`, `Ord`
/// - `Serialize`, `Deserialize` (if using `serde`)
///
/// Example with multiple traits:
/// ```rust,ignore
/// use dxc_macros::props;
/// use serde::{Serialize, Deserialize};
/// 
/// props! {
///     Config {
///         timeout: u64,
///         enabled: bool,
///     },
///     derive: [Default, Serialize, Deserialize]
/// }
/// ```
///
/// # See Also
///
/// Consider using [`#[derive(Default)]`](https://doc.rust-lang.org/std/default/trait.Default.html)
/// in combination with this macro for convenient initialization.
#[macro_export]
macro_rules! props {
    (
        $name:ident {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $name {
            $(
                pub $field: Option<$type>
            ),*
        }
    };

    (
        $name:ident {
            $($field:ident : $type:ty),* $(,)?
        },
        derive: [$($extra:ident),*]
    ) => {
        #[derive(Clone, PartialEq, Debug, $($extra),*)]
        pub struct $name {
            $(
                pub $field: Option<$type>
            ),*
        }
    };
}
