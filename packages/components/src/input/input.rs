use super::props::InputProps;
use crate::DxcIcon;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_icons::{spawn_icon, CircleClose, Hide, View};
use dxc_macros::classes;
use dxc_types::namespace::Block;

#[component]
pub fn DxcInput(props: InputProps) -> Element {
    // State
    let input_id = use_signal(|| props.id());
    let input_type = props.type_.clone().unwrap_or("text".to_string());
    let input_resize = use_signal(|| props.resize().to_string());
    let input_size = use_signal(|| props.size().to_string());

    let mut input_value = props.value();
    let input_disable = use_signal(|| props.disabled());

    let clearable = use_signal(|| props.clearable());
    let read_only = use_signal(|| props.read_only());
    let show_word_limit = use_signal(|| props.show_word_limit());
    let show_password = use_signal(|| props.show_password());

    let validate_state = use_signal(|| String::new());

    let need_status_icon = use_signal(|| false);

    let mut is_focused = use_signal(|| false);

    let mut hovering = use_signal(|| false);
    let mut password_visible = use_signal(|| false);

    let show_clear = use_memo(move || {
        clearable() && !input_disable() && !read_only() && !input_value().is_empty()
        // && (is_focused() || hovering())
    });

    let show_pwd_visible =
        use_signal(|| show_password() && !input_disable() && input_value().is_empty());

    let is_word_limit_visible = use_signal(|| {
        show_word_limit()
            && !!props.max_length.is_some()
            && (props.type_.as_deref() == Some("text")
                || props.type_.as_deref() == Some("textarea"))
            && !input_disable()
            && !read_only()
            && !show_password()
    });
    let text_length = use_signal(move || input_value().chars().count());
    let input_exceed =
        use_signal(|| !!is_word_limit_visible() && (text_length() > props.max_length.unwrap_or(0)));

    let is_suffix = props.suffix.is_some();

    let suffix_visible = use_memo(move || {
        is_suffix
            // || !!props.suffix.is_some()
            || show_clear()
            || show_password()
            || is_word_limit_visible()
            || (validate_state().is_empty() && need_status_icon())
    });

    // Styles
    let ns_textarea = UseNamespace::new(Block::Textarea, None);
    let ns_input = UseNamespace::new(Block::Input, None);

    let container_classes = classes! {
        if props.type_ == Some("textarea".to_string()) {&ns_textarea.b()} else {&ns_input.b();},
        &ns_input.m_(input_size()),
        &ns_input.is_(String::from("disabled"), Some(input_disable())),
        &ns_input.is_(String::from("exceed"), Some(input_exceed())),

        if props.append.is_some() || props.prepend.is_some() {&ns_input.b_(String::from("group"))},
        if props.prepend.is_some() || props.prefix.is_some() {&ns_input.m_(String::from("prefix"))},
        if props.suffix.is_some() || props.suffix.is_some() || clearable() || show_password() {&ns_input.m_(String::from("suffix"))},
        if show_clear() && show_pwd_visible() {&ns_input.bm_(String::from("suffix"),String::from("password-clear"))},

        if props.append.is_some() { &ns_input.bm_(String::from("group"), String::from("append")) },
        if props.prepend.is_some() { &ns_input.bm_(String::from("group"), String::from("prepend")) },

        &props.class.as_deref().unwrap_or(""),
    };
    let wrapper_classes = classes! {
        ns_input.e_(String::from("wrapper")),
        ns_input.is_(String::from("focus"), Some(is_focused()))
    };

    let textarea_calc_style = format!("");
    let textarea_style = format!(
        "{} {} {}",
        props.input_style.unwrap_or(String::new()),
        textarea_calc_style,
        input_resize()
    );

    rsx! {
        div {
            id: input_id,
            class:container_classes,

            if "textarea" != input_type{

                if props.prepend.is_some() {
                    div {
                        class:ns_input.be_(String::from("group"), String::from("prepend")),
                        {props.prepend}
                    }
                }
                div {
                    class: wrapper_classes,
                    // prefix slot
                    if props.prefix.is_some() || props.prefix_icon.is_some() {
                        span {
                            class: ns_input.e_(String::from("prefix")),
                            span {
                                class: ns_input.e_(String::from("prefix-inner")),
                                {props.prefix}
                                DxcIcon {
                                    children: spawn_icon(props.prefix_icon.as_deref().unwrap_or("")),
                                }
                            }
                        }
                    }

                    input {
                        class: ns_input.e_(String::from("inner")),
                        name: props.name,
                        minlength: props.minlength,
                        maxlength: props.max_length,
                        r#type: match (show_password(), password_visible()) {
                            (true, true) => "text".to_string(),
                            (true, false) => "password".to_string(),
                            (false, _) => input_type,
                        },
                        disabled: input_disable(),
                        readonly: read_only(),
                        value: "{input_value()}",
                        autocomplete: props.auto_complete,
                        tabindex: props.tab_index,
                        aria_label: props.aria_label,
                        placeholder: props.placeholder,
                        style: props.style,
                        form: props.form,
                        autofocus: props.autofocus,
                        role: props.container_role,
                        inputmode: props.inputmode,
                        oninput: move |envent | {
                            input_value.set(envent.value());
                            props.oninput.clone().unwrap_or_default();
                        },
                        onfocus: move |_| {
                            is_focused.set(true);
                            props.onfocus.clone().unwrap_or_default();
                        },
                        onblur: move |_| {
                            is_focused.set(false);
                            props.onblur.clone().unwrap_or_default();
                        },
                        onmouseover: move |_| hovering.set(true),
                        onmouseleave: move |_| hovering.set(false),
                        onchange: props.onchange.clone().unwrap_or_default(),
                        onkeydown: props.onkeydown.clone().unwrap_or_default(),
                    }

                    // suffix slot
                    if suffix_visible() || props.suffix_icon.is_some() {
                        span {
                            class: ns_input.e_(String::from("suffix")),
                            span {
                                class: ns_input.e_(String::from("suffix-inner")),

                                if !show_clear() || !show_pwd_visible() || !is_word_limit_visible() {
                                    {props.suffix}
                                    DxcIcon {
                                        class: ns_input.e_(String::from("icon")),
                                        children: spawn_icon(props.suffix_icon.as_deref().unwrap_or(""))
                                    }
                                }

                                if show_clear() {
                                    DxcIcon {
                                        class: format!("{} {}", ns_input.e_(String::from("icon")), ns_input.e_(String::from("clear"))),
                                        onclick: move |event:MouseEvent| {
                                            event.prevent_default();
                                            input_value.set(String::new());
                                        },
                                        CircleClose { }
                                    }
                                }

                                if show_pwd_visible() {
                                    DxcIcon {
                                        class: format!("{} {}", ns_input.e_(String::from("icon")),ns_input.e_(String::from("password"))),
                                        onclick: move |_| {
                                            password_visible.set(!password_visible());
                                        },
                                        if password_visible() {
                                            View {}
                                        } else {
                                            Hide {}
                                        },
                                    }
                                }
                            }
                        }
                    }
                }

                // append slot
                if props.append.is_some(){
                    div {
                        class: ns_input.be_(String::from("group"), String::from("append")),
                        {props.append}
                    }
                }
            } else {

                div {
                    textarea {
                        style: textarea_style,
                    }
                }
            }
        }
    }
}
