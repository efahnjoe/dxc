use super::action::Action;
use super::props::ImageViewerProps;
use super::transform::Transform;
use crate::components::focus_trap::DxcFocusTrap;
use crate::components::icon::DxcIcon;
use crate::components::teleport::DxcTeleport;
use crate::components::transition::DxcTransition;
use crate::hooks::UseNamespace;
use crate::types::namespace::Block;
use dioxus::prelude::*;
use dxc_icons::{
    ArrowLeft, ArrowRight, Close, FullScreen, RefreshLeft, RefreshRight, ScaleToOriginal, ZoomIn,
    ZoomOut,
};
use dxc_macros::classes;

#[component]
pub fn DxcImageViewer(props: ImageViewerProps) -> Element {
    let mut active_index = use_signal(|| props.initial_index());
    let current_index = use_memo(move || active_index());
    let crossorigin = use_signal(|| props.crossorigin());
    let mut loading = use_signal(|| true);
    let hide_on_click_modal = use_signal(|| props.hide_on_click_modal());
    let infinite = use_signal(|| props.infinite());
    let transform = use_signal(|| Transform::default());

    let zoom_rate = use_signal(|| props.zoom_rate());
    let min_scale = use_signal(|| props.min_scale());
    let max_scale = use_signal(|| props.max_scale());
    let z_index = use_signal(|| props.z_index());

    let url_list = use_signal(|| props.url_list());
    let is_single = use_signal(|| {
        return url_list().iter().len() <= 1;
    });

    let is_first = use_signal(|| active_index() == 0);
    let is_last = use_signal(|| active_index() == url_list().len().saturating_sub(1));

    let show_progress = use_signal(|| props.show_progress());
    let progress = use_memo(move || format!("{} / {}", active_index() + 1, url_list.len()));

    // style
    let ns = UseNamespace::new(Block::ImageViewer, None);

    let arrow_pre_classes = classes! {
        ns.e_(String::from("btn")),
        ns.e_(String::from("prev")),
        ns.is_(String::from("disabled"), Some(!infinite() && is_first())),
    };
    let arrow_next_classes = classes! {
        ns.e_(String::from("btn")),
        ns.e_(String::from("next")),
        ns.is_(String::from("disabled"), Some(!infinite() && is_last())),
    };
    let img_style = use_memo(move || {
        let t = transform();
        let scale = t.scale;
        let deg = t.deg;
        let enable_transition = t.enable_transition;
        let translate_x = t.offset_x / scale;
        let translate_y = t.offset_y / scale;

        if scale == 0.0 {
            return "transform: scale(0);".to_string();
        }

        let radian = (deg * std::f64::consts::PI) / 180.0;
        let cos_radian = radian.cos();
        let sin_radian = radian.sin();

        let new_translate_x = translate_x * cos_radian + translate_y * sin_radian;
        let new_translate_y = translate_y * cos_radian - translate_x * sin_radian;

        let style = format!(
            "transform: scale({scale}) rotate({deg}deg) translate({new_translate_x}px, {new_translate_y}px); transition: {}; max-width: 100%; max-height: 100%;",
            if enable_transition {
                "transform 0.3s"
            } else {
                "transform 0.0s"
            }
        );

        style
    });

    // event
    // let regisiter_event_listener = move || {
    //     let key_down_handler = move |evt: Event<KeyboardData>| match evt.key() {
    //         Key::Escape => props.close_on_press_escape.unwrap_or(false),
    //         _ => false,
    //     };
    // };

    let mut set_active_item = move |index: usize| {
        let len = url_list().len();

        if len == 0 {
            return;
        }

        active_index.set((index + len) % len);
    };

    let mut prev = move || {
        let len = url_list().len();

        if is_first() && !infinite() {
            return;
        }

        set_active_item(active_index().checked_sub(1).unwrap_or(len - 1));
    };

    let mut next = move || {
        let len = url_list().len();

        if is_last() && !infinite() {
            return;
        }

        set_active_item(active_index().checked_add(1).unwrap_or(len + 1));
    };

    let hide = move |evt: Event<MouseData>| {
        evt.stop_propagation();
        if let Some(onclose_handler) = props.onclose {
            onclose_handler.call(());
        }
    };

    let mut handle_actions = {
        let mut transform = transform.clone();

        move |action: Action| {
            if loading() {
                return;
            }
            let mut t = transform.write();

            let rotate_deg = 90.0;
            let enable_transition = true;

            match action {
                Action::ZoomIn => {
                    if t.scale > min_scale() {
                        t.scale *= zoom_rate();
                    }
                    println!("Action: {:?}", action);
                }
                Action::ZoomOut => {
                    if t.scale < max_scale() {
                        t.scale /= zoom_rate();
                    }
                }
                Action::Clockwise => {
                    t.deg += rotate_deg;
                }
                Action::Anticlockwise => {
                    t.deg -= rotate_deg;
                }
            }

            t.enable_transition = enable_transition;
        }
    };

    let mut toggle_mode_icon = use_signal(|| false);

    let mut toogle_mode = move || {
        if loading() {
            return;
        }
        toggle_mode_icon.set(!toggle_mode_icon());
    };

    let slot_progress = props.progress.clone();
    let slot_toolbar = props.toolbar.clone();

    rsx! {
        DxcTeleport {
            DxcTransition {
                div {
                    tabindex: "-1",
                    z_index: z_index(),
                    class: ns.e_(String::from("wrapper")),
                    DxcFocusTrap{
                        loop_: true,
                        trapped: true,
                        div {
                            class: ns.e_(String::from("mask")),
                            onclick: move |evt| {
                                if hide_on_click_modal() {
                                    hide(evt)
                                }
                            }
                        }

                        //close
                        span {
                            class: format!("{} {}",ns.e_(String::from("btn")),ns.e_(String::from("close"))),
                            onclick: move |evt| {
                                hide(evt)
                            },
                            DxcIcon { Close { } }
                        }

                        // arrow
                        if !is_single() {
                            span {
                                class: arrow_pre_classes,
                                onclick: move |_| { prev() },
                                DxcIcon { ArrowLeft { } }
                            }
                            span {
                                class: arrow_next_classes,
                                onclick: move |_| { next() },
                                DxcIcon { ArrowRight {  } }
                            }
                        }

                        if slot_progress.is_some() || show_progress(){
                            div {
                                class: format!("{} {}", ns.e_(String::from("btn")), ns.e_(String::from("progress"))),
                                {slot_progress}
                                {progress()}
                            }
                        }

                        // actions
                        div {
                            class: format!("{} {}", ns.e_(String::from("btn")), ns.e_(String::from("actions"))),

                            div {
                                class: ns.e_(String::from("actions__inner")),
                                {slot_toolbar}

                                DxcIcon {
                                    onclick: move |_| handle_actions(Action::ZoomOut),
                                    ZoomOut {  }
                                }
                                DxcIcon {
                                    onclick: move |_| handle_actions(Action::ZoomIn),
                                    ZoomIn {  }
                                }
                                i { class:ns.e_(String::from("actions__divider")) }
                                DxcIcon {
                                    onclick: move |_| toogle_mode(),
                                    if toggle_mode_icon() {
                                        ScaleToOriginal {  }
                                    } else {
                                        FullScreen {  }
                                    }
                                }
                                i { class:ns.e_(String::from("actions__divider")) }
                                DxcIcon {
                                    onclick: move |_| handle_actions(Action::Anticlockwise),
                                    RefreshLeft {  }
                                }
                                DxcIcon {
                                    onclick: move |_| handle_actions(Action::Clockwise),
                                    RefreshRight {  }
                                }
                            }
                        }

                        // canvas
                        div {
                            class:ns.e_(String::from("canvas")),
                            for (index, item) in url_list().iter().enumerate() {
                                if index == current_index() {
                                    img {
                                        key: "{index}",
                                        src: "{item}",
                                        class: ns.e_(String::from("img")),
                                        style: img_style,
                                        crossorigin: crossorigin(),
                                        onload: move |_| {
                                            loading.set(false);
                                        },
                                        onerror: move |_| {
                                            loading.set(false);
                                        },
                                        onmousedown: move |evt| {
                                            evt.prevent_default();
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
