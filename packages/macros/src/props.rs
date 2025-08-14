// use dioxus::prelude::props;

#[macro_export]
macro_rules! props {
    // 基础版
    (
        $name:ident {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $name {
            $(
                // #[props(default)]
                pub $field: Option<$type>
            ),*
        }
    };

    // 扩展版：支持额外 derive
    (
        $name:ident {
            $($field:ident : $type:ty),* $(,)?
        },
        derive: [$($extra:ident),*]
    ) => {
        #[derive(Clone, PartialEq, Debug, $($extra),*)]
        pub struct $name {
            $(
                #[props(default)]
                pub $field: Option<$type>
            ),*
        }
    };
}
