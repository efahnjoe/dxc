use crate::DxcIcon;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_icons::spawn_icon;
use dxc_macros::{classes, props};

props! {
    InputProps {
        id: String,
        type_: String,
        size: String,
        disabled: bool,
        exceed: bool,
        model_value: String,
        minlength: usize,
        max_length: usize,
        resize: String,
        auto_size: String,
        auto_complete: String,
        formatter: fn(),
        parser: fn(),
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
        append: Element,
        prepend: Element,
        prefix: Element,
        suffix: Element,
        prefix_icon: String,
        suffix_icon: String,

        // Default Props
        class: String,
        style: String,
        children: Element,
    }
}

#[component]
pub fn DxcInput(props: InputProps) -> Element {
    // State
    let input_type = use_signal(|| match props.type_.as_deref() {
        Some("textarea") => "textarea".to_string(),
        _ => "input".to_string(),
    });
    let mut input_value = use_signal(|| props.model_value.clone().unwrap_or(String::new()));
    let input_disable = use_signal(|| props.disabled.unwrap_or(false).clone());

    let clearable = use_signal(|| props.clearable.unwrap_or(false));
    let read_only = use_signal(|| props.read_only.unwrap_or(false));
    let show_word_limit = use_signal(|| props.show_word_limit.unwrap_or(false));
    let show_password = use_signal(|| props.show_password.unwrap_or(false));

    let native_input_value = use_signal(|| props.model_value.clone());

    let validate_state = use_signal(|| Some(String::new()));

    let need_status_icon = use_signal(|| false);

    let is_focused = use_signal(|| false);
    let hovering = use_signal(|| false);
    let show_clear = use_signal(|| {
        clearable()
            && !input_disable()
            && !read_only()
            && !!native_input_value().is_some()
            && (is_focused() || hovering())
    });
    let mut password_visible = use_signal(|| false);

    let show_pwd_visible =
        use_signal(|| show_password() && !input_disable() && !!native_input_value().is_some());
    let is_word_limit_visible = use_signal(|| {
        show_word_limit()
            && !!props.max_length.is_some()
            && (props.type_.as_deref() == Some("text")
                || props.type_.as_deref() == Some("textarea"))
            && !input_disable()
            && !read_only()
            && !show_password()
    });
    let text_length = use_signal(|| {
        native_input_value
            .read()
            .as_ref()
            .map(|s| s.len())
            .unwrap_or(0)
    });
    let input_exceed =
        use_signal(|| !!is_word_limit_visible() && text_length() > props.max_length.unwrap_or(0));
    let suffix_visible = use_signal(|| {
        !!props.suffix.is_some()
            || !!props.suffix.is_some()
            || show_clear()
            || show_password()
            || is_word_limit_visible()
            || (!!validate_state().is_some() && need_status_icon())
    });

    // Events
    // let focus_ctl = use_focus_controller(UseFocusControllerOptions {
    //     disabled: input_disable(),
    //     before_blur: Some(EventHandler::new(move |event: FocusEvent| {
    //         if Some(true) == props.validate_event {
    //             // props.validate_event(event);
    //         }
    //     })),
    //     ..Default::default()
    // });

    // Styles
    let ns_textarea = UseNamespace::new("textarea", None);
    let ns_input = UseNamespace::new("input", None);

    let container_classes = classes! {
        if props.type_ == Some("textarea".to_string()) {&ns_textarea.b()} else {&ns_input.b();},
        &ns_input.m_(props.size.as_deref().unwrap_or("")),
        &ns_input.is_("disabled", Some(input_disable())),
        &ns_input.is_("exceed", Some(input_exceed())),

        if props.append.is_some() || props.prepend.is_some() {&ns_input.b_("group")},
        if props.prepend.is_some() || props.prefix.is_some() {&ns_input.m_("prefix")},
        if props.suffix.is_some() || props.suffix.is_some() || clearable() || show_password() {&ns_input.m_("suffix")},
        if show_clear() && show_pwd_visible() {&ns_input.bm_("suffix","password-clear")},

        if props.append.is_some() { &ns_input.bm_("group", "append") },
        if props.prepend.is_some() { &ns_input.bm_("group", "prepend") },

        &props.class.as_deref().unwrap_or(""),
    };
    let wrapper_classes = classes! {
        ns_input.e_("wrapper"),
        ns_input.is_("focus", Some(is_focused()))
    };

    rsx! {
        div {
            class:container_classes,

            if "textarea".to_string() != input_type(){

                if props.prepend.is_some() {
                    div {
                        class:ns_input.be_("group", "prepend"),
                        {props.prepend}
                    }
                }
                div {
                    class: wrapper_classes,
                    // prefix slot
                    if props.prefix.is_some() || props.prefix_icon.is_some() {
                        span {
                            class: ns_input.e_("prefix"),
                            span {
                                class: ns_input.e_("prefix-inner"),
                                {props.prefix}
                                DxcIcon {
                                    children: spawn_icon(props.prefix_icon.as_deref().unwrap_or("")),
                                }
                            }
                        }
                    }

                    input {
                        class: ns_input.e_("inner"),
                        name: props.name,
                        minlength: props.minlength,
                        maxlength: props.max_length,
                        r#type: match (show_password(), password_visible()) {
                            (true, true) => "text".to_string(),
                            (true, false) => "password".to_string(),
                            (false, _) => input_type(),
                        },
                        disabled: input_disable(),
                        readonly: read_only(),
                        value: input_value(),
                        autocomplete: props.auto_complete,
                        tabindex: props.tab_index,
                        aria_label: props.aria_label,
                        placeholder: props.placeholder,
                        style: props.style,
                        form: props.form,
                        autofocus: props.autofocus,
                        role: props.container_role,
                        inputmode: props.inputmode,
                        // oncompositionstart
                        oninput: props.oninput.clone().unwrap_or_default(),
                        onfocus: props.onfocus.clone().unwrap_or_default(),
                        onblur: props.onblur.clone().unwrap_or_default(),
                        onchange: props.onchange.clone().unwrap_or_default(),
                        onkeydown: props.onkeydown.clone().unwrap_or_default(),
                    }

                    // suffix slot
                    if suffix_visible() || props.suffix_icon.is_some() {
                        span {
                            class: ns_input.e_("suffix"),
                            span {
                                class: ns_input.e_("suffix-inner"),

                                if !show_clear() || !show_pwd_visible() || !is_word_limit_visible() {
                                    {props.suffix}
                                    DxcIcon {
                                        class: ns_input.e_("icon"),
                                        children: spawn_icon(props.suffix_icon.as_deref().unwrap_or(""))
                                    }
                                }

                                if show_clear() {
                                    DxcIcon {
                                        class: format!("[{} {}]", ns_input.e_("icon"), ns_input.e_("clear")),
                                        onclick: move |_| {
                                            input_value.set(String::new());
                                        },
                                        children: spawn_icon("CircleClose")
                                    }
                                }

                                if show_pwd_visible() {
                                    DxcIcon {
                                        class: format!("{} {}", ns_input.e_("icon"),ns_input.e_("password")),
                                        onclick: move |_| {
                                            password_visible.set(!password_visible());
                                        },
                                        // children: rsx!{if show_password_visible(){spawn_icon("View")} else {spawn_icon("Hide")}}
                                        children: spawn_icon("View")
                                    }
                                }
                            }
                        }
                    }
                }

                // append slot
                if props.append.is_some(){
                    div {
                        class: ns_input.be_("group", "append"),
                        {props.append}
                    }
                }
            } else {

                div {
                    textarea {

                    }
                }
            }
        }
    }
}
