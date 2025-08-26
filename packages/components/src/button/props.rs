use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};
use dxc_types::components::button::*;

props! {
    ButtonProps {
        #[props_default(value = Size::Default)]
        size: Size,
        disabled: bool,
        
        #[props_default(value = Type::Primary)]
        type_: Type,

        icon: String,
        navtive_type: String,
        loading: bool,
        loading_icon: String,
        plain: bool,
        text: bool,
        link: bool,
        bg: bool,
        autofocus: bool,
        round: bool,
        circle: bool,
        color: String,
        dark: bool,
        auto_insert_space: bool,
        tag: Vec<String>,
        onclick: EventHandler<MouseEvent>,

        class: String,
        style: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}

props! {
    ButtonGroupProps {
        #[props_default(value = Size::Default)]
        size: Size,

        #[props_default(value = Type::Primary)]
        type_: Type,


        class: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
