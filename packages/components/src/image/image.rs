use super::props::ImageProps;
use crate::image_viewer::DxcImageViewer;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_macros::classes;
use dxc_types::{Loading,namespace::Block};

#[component]
pub fn DxcImage(props: ImageProps) -> Element {
    let img_src = use_signal(|| props.src());
    let alt = use_signal(|| props.alt());
    let loading = use_signal(|| props.loading());
    let lazy = use_signal(|| props.lazy());
    let fit = use_signal(|| props.fit());
    let initial_index = use_signal(|| props.initial_index());
    let preview_src_list = use_signal(|| props.preview_src_list());
    let crossorigin = use_signal(|| props.crossorigin());

    let mut image_src = use_signal(|| String::new());

    let mut has_load_error = use_signal(|| false);
    let mut is_loading = use_signal(|| true);
    let mut show_viewer = use_signal(|| false);
    let support_loading = use_signal(|| false);
    let preview = use_signal(|| preview_src_list.len() > 0);

    let image_index = use_memo(move || {
        let mut preview_index = initial_index();

        if let Some(next) = preview_src_list().len().checked_sub(1) {
            if initial_index() > next {
                preview_index = 0;
            }
        }

        preview_index
    });

    let is_manual = use_signal(|| {
        if loading() == Loading::Lazy {
            return false;
        }

        return !support_loading() && loading() == Loading::Lazy || lazy();
    });

    let mut laod_image = move || {
        is_loading.set(true);
        has_load_error.set(false);
        image_src.set(img_src());
    };

    let handle_lazy_load = move || {
        laod_image();
    };

    // use_before_render(|| {
    //     if is_manual() {
    //         handle_lazy_load();
    //     }
    // });

    let ns = UseNamespace::new(Block::Image, None);

    let image_classes = classes! {
        ns.e_(String::from("inner")),
        if preview() { &ns.e_(String::from("preview")) },
        if is_loading() { &ns.e_(String::from("loading")) },
    };

    let image_style = format!("object-fit: {};", fit());

    rsx! {
        div {
            class: ns.b(),

            // if image load error, show error message slot
            // else show image
            if has_load_error() {
                div {
                    "name": "image_error",
                    div {
                        class: ns.e_(String::from("error")),
                        "FAILED"
                    }
                    {props.error}
                }
            } else {
                div {
                    // if image src is set, show image
                    // else show placeholder
                    if !img_src().is_empty(){
                        img {
                            src: img_src(),
                            alt: alt(),
                            class: image_classes,
                            style: image_style,
                            loading: loading(),
                            crossorigin: crossorigin(),
                            onclick: move |_| {
                                if !preview() {
                                    return;
                                }

                                show_viewer.set(true);
                            },
                            onload: move |_| {
                                is_loading.set(false);
                                has_load_error.set(false);
                            },
                            onerror: move |_| {
                                is_loading.set(false);
                                has_load_error.set(true);
                            },
                        }
                    }

                    if is_loading() {
                        div {
                            class: ns.e_(String::from("wrapper")),
                            div {
                                class: ns.e_(String::from("placeholder")),
                                {props.placeholder}
                            }
                        }
                    }
                }
            }

            if preview() {
                if show_viewer() {
                    DxcImageViewer{
                        z_index: props.z_index,
                        initial_index: image_index(),
                        infinite: props.infinite,
                        zoom_rate: props.zoom_rate,
                        min_scale: props.min_scale,
                        max_scale: props.max_scale,
                        show_progress: props.show_progress,
                        url_list: preview_src_list(),
                        crossorigin: crossorigin(),
                        // hide_on_cllick_modal: hide_on_click_modal,
                        // teleported: preview_teleported(),
                        // close_on_press_escape: close_on_press_escape,
                        onclose: move |_| {
                            show_viewer.set(false);
                        },
                        // on_switch: move |index| {
                        //     // current_index.set(index);
                        // },
                    }
                }
            }
        }
    }
}
