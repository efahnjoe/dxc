use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};
use dxc_types::Crossorigin;

props! {
    ImageViewerProps {
        url_list: Vec<String>,

        #[props_default(value = 2000)]
        z_index: i32,

        #[props_default(value = 0)]
        initial_index: usize,

        infinite: bool,

        hide_on_click_modal: bool,

        teleported: bool,

        close_on_press_escape: bool,

        #[props_default(value = 1.2)]
        zoom_rate: f64,

        #[props_default(value = 0.2)]
        min_scale: f64,

        #[props_default(value = 7.0)]
        max_scale: f64,

        show_progress: bool,

        #[props_default(value = Crossorigin::Anonymous)]
        crossorigin: Crossorigin,

        onclose: EventHandler<()>,

        switch: EventHandler<MouseEvent>,

        #[props_default(skip)]
        progress: Element,

        #[props_default(skip)]
        toolbar: Element,
    },
    derive(PropsDefault)
}
