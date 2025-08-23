use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};
use dxc_types::{Crossorigin, Fit, Loading};

props! {
    ImageProps {
        /// When enabling preview,
        /// use this flag to control whether clicking on backdrop can exit preview mode.
        hiden_on_click_modal: bool,

        /// Image source, same ad native.
        src: String,

        /// Indicate how the image should be resized to fit its container,
        /// same as [object-fit](https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit).
        #[props_default(value = Fit::None)]
        fit: Fit,

        /// Indicates how the browser should load the image,
        /// same as [native](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#attr-loading).
        #[props_default(value = Loading::Lazy)]
        loading: Loading,

        /// Whether to use lazy load.
        lazy: bool,

        referrerpolicy: String,

        /// Allow big image preview
        preview_src_list: Vec<String>,

        /// whether to append image-viewer to body.
        /// A nested parent element attribute transform should have this attribute set to `true`.
        preview_teleported: bool,

        z_index: i32,

        initial_index: usize,

        infinite: bool,

        close_on_press_escape: bool,

        zoom_rate: f64,

        min_scale: f64,

        max_scale: f64,

        show_progress: bool,

        #[props_default(value = Crossorigin::Anonymous)]
        crossorigin: Crossorigin,

        /// Image alt text.
        alt: String,

        class: String,

        style: String,

        /// the container to add scroll listener when using lazy load.
        #[props_default(skip)]
        sroll_container: Element,

        #[props_default(skip)]
        error: Element,

        #[props_default(skip)]
        placeholder: Element,

        #[props_default(skip)]
        viewer: Element,

        #[props_default(skip)]
        progress: Element,

        #[props_default(skip)]
        toolbar: Element,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
