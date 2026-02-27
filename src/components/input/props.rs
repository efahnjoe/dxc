use crate::types::components::input::{Resize, Size};
use dioxus::prelude::*;
use dxc_macros::{PropsDefault, props};

props! {
    InputProps {
        id: String,
        type_: String,

        #[props_default(value = Size::Default)]
        size: Size,
        disabled: bool,
        exceed: bool,
        value: Signal<String>,
        minlength: usize,
        max_length: usize,

        #[props_default(value = Resize::None)]
        resize: Resize,

        auto_size: String,
        auto_complete: String,
        formatter: EventHandler<()>,
        parser: EventHandler<()>,
        placeholder: String,
        read_only: bool,
        clearable: bool,
        show_password:bool,
        show_word_limit: bool,
        container_role: String,
        tab_index: usize,
        validate_event: bool,
        input_style: String,
        rows: usize,
        inputmode: String,
        name: String,

        // events
        onfocus: EventHandler<FocusEvent>,
        onblur: EventHandler<FocusEvent>,
        onchange: EventHandler<FormEvent>,
        oninput: EventHandler<FormEvent>,
        onkeydown: EventHandler<KeyboardEvent>,

        // native
        aria_label: String,
        form: String,
        autofocus: bool,

        // Slots
        #[props_default(skip)]
        append: Element,

        #[props_default(skip)]
        prepend: Element,

        #[props_default(skip)]
        prefix: Element,

        #[props_default(skip)]
        suffix: Element,

        prefix_icon: String,
        suffix_icon: String,

        // Default Props
        class: String,
        style: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
