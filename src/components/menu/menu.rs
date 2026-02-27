use dioxus::prelude::*;

#[component]
pub fn DxcMenu(children: Element, class: Option<String>) -> Element {
    let combined_class = format!("menu {}", class.unwrap_or_default());

    rsx! {
        ul {
            role: "menu",
            class: combined_class,
            tabindex: "-1",
            { children }
        }
    }
}

// use super::props::MenuProps;
// use dioxus::prelude::*;

// #[component]
// pub fn DxcMenu(props: MenuProps) -> Element {
//     let combined_class = format!("menu {}", props.class());

//     rsx! {
//         ul {
//             role: "menu",
//             class: combined_class,
//             tabindex: "-1",
//             { props.children }
//         }
//     }
// }

// #[component]
// pub fn DxcMenuItem(
//     children: Element,
//     class: Option<String>,
//     disabled: Option<bool>,
//     index: Option<u32>,
//     route: Option<&'static str>,
// ) -> Element {
//     let disabled = disabled.unwrap_or(false);
//     let index = index.unwrap_or(0);
//     let mut classes = format!("menu-item {}", class.unwrap_or_default());

//     if disabled {
//         classes += " menu-item-disabled";
//     }

//     rsx! {
//         li {
//             class: classes,
//             "index": index,
//             tabindex: "-1",
//             aria_disabled: disabled.then(|| "true"),

//             if !disabled {
//                 if let Some(target_route) = route {
//                     Link {
//                         to: target_route,
//                         { children }
//                     }
//                 } else {
//                     { children }  // 没有路由时直接显示
//                 }
//             } else {
//                 { children }  // 禁用时不响应点击
//             }
//         }
//     }
// }
