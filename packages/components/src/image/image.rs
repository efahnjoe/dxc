use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use std::collections::HashMap;
use std::iter::{empty, once};

#[derive(Clone, Props, PartialEq)]
pub struct ImageProps {
    /// When enabling preview,
    /// use this flag to control whether clicking on backdrop can exit preview mode.
    #[props(default)]
    hiden_on_click_modal: Option<bool>,

    // Image source, same ad native.
    #[props(default)]
    src: Option<String>,

    /// Indicate how the image should be resized to fit its container,
    /// same as [object-fit](https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit).
    #[props(default)]
    fit: Option<String>,

    /// Indicates how the browser should load the image,
    /// same as [native](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#attr-loading).
    #[props(default)]
    loading: Option<String>,

    /// Whether to use lazy load.
    #[props(default)]
    lazy: Option<bool>,

    /// the container to add scroll listener when using lazy load.
    #[props(default)]
    sroll_container: Option<Element>,

    #[props(default)]
    referrerpolicy: Option<String>,

    /// Allow big image preview
    #[props(default)]
    preview_src_list: Option<HashMap<String, String>>,

    /// whether to append image-viewer to body.
    /// A nested parent element attribute transform should have this attribute set to `true`.
    #[props(default)]
    preview_teleported: Option<bool>,

    #[props(default)]
    z_index: Option<i32>,

    #[props(default)]
    initial_index: Option<usize>,

    #[props(default)]
    infinite: Option<bool>,

    #[props(default)]
    close_on_press_escape: Option<bool>,

    #[props(default)]
    zoom_rate: Option<f64>,

    #[props(default)]
    min_scale: Option<f64>,

    #[props(default)]
    max_scale: Option<f64>,

    #[props(default)]
    show_progress: Option<bool>,

    #[props(default)]
    crossorigin: Option<String>,

    /// Image alt text.
    #[props(default)]
    alt: Option<String>,

    #[props(default)]
    class: Option<String>,

    #[props(default)]
    style: Option<String>,

    #[props(default)]
    image_error: Option<Element>,

    #[props(default)]
    image_placeholder: Option<Element>,

    #[props(default)]
    image_viewer: Option<Element>,

    #[props(default)]
    image_progress: Option<Element>,

    #[props(default)]
    image_toolbar: Option<Element>,

    #[props(default)]
    children: Option<Element>,
}

#[component]
pub fn DxcImage(props: ImageProps) -> Element {
    let ImageProps {
        // Image Attributes
        src,
        alt,
        class,
        style,
        fit,
        hiden_on_click_modal,
        loading,
        lazy,
        sroll_container,
        referrerpolicy,
        crossorigin,
        preview_src_list,
        z_index,
        initial_index,
        close_on_press_escape,
        preview_teleported,
        infinite,
        zoom_rate,
        min_scale,
        max_scale,
        show_progress,

        // Image Slots
        image_error,
        image_placeholder,
        image_viewer,
        image_progress,
        image_toolbar,
        children,
    } = props;

    let ns = UseNamespace::new("image", None);

    let mut has_load_error = use_signal(|| false);
    let mut is_loading = use_signal(|| true);
    let show_viewer = use_signal(|| false);
    let container = rsx!();
    let scroll_container = rsx!();
    let preview = use_signal(|| match &preview_src_list {
        Some(map) => !map.is_empty(),
        None => false,
    });

    let image_class = empty()
        .chain(once(ns.e_("inner")))
        .chain(preview().then(|| ns.e_("preview")))
        .chain(is_loading().then(|| ns.e_("loading")))
        .collect::<Vec<_>>()
        .join(" ");

    let image_index = use_signal(move || match (&initial_index, &preview_src_list) {
        (Some(idx), Some(map)) if *idx < map.len() => *idx,
        _ => 0,
    });

    rsx! {
        div {
            class: ns.b(),

            // if image load error, show error message slot
            // else show image
            if has_load_error() {
                div {
                    "name": "image_error",
                    div {
                        class: ns.e_("error"),
                        "Image failed to load"
                    }
                    {image_error}
                }
            } else {
                div {
                    // if image src is set, show image
                    // else show placeholder
                    if src.is_some(){
                        img {
                            src: src,
                            alt: alt,
                            class: image_class,
                            loading: loading.as_deref().unwrap_or("lazy"),
                            crossorigin: crossorigin.as_deref().unwrap_or("anonymous"),
                            onclick: move |_| {
                                // set_load_error(false);
                            },
                            onload: move |_| {
                                is_loading.set(false);
                            },
                            onerror: move |_| {
                                has_load_error.set(true);
                            },
                        }
                    }

                    if is_loading() {
                        div {
                            class: ns.e_("wrapper"),
                            div {
                                "name": "image_placeholder",
                                class: ns.e_("placeholder"),
                                {image_placeholder}
                            }
                        }
                    }
                }
            }

            if preview() {
                // ImageViewer{
                //     z_index: z_index.as_deref(),
                //     inintial_index: image_index,
                //     infinite: infinite.as_deref(),
                //     zoom_rate: zoom_rate.as_deref(),
                //     min_scale: min_scale.as_deref(),
                //     max_scale: max_scale.as_deref(),
                //     show_progress: show_progress,
                //     url_list: preview_src_list.clone(),
                //     crossorigin: crossorigin.as_deref(),
                //     hide_on_cllick_modal: hide_on_click_modal,
                //     teleproted: preview_teleported,
                //     close_on_press_escape: close_on_press_escape,
                //     on_close: move |_| {
                //         show_viewer.set(false);
                //     },
                //     on_switch: move |index| {
                //         // current_index.set(index);
                //     },
                // }
            }
        }
    }
}
