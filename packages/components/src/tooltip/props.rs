use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

// type StateUpdater = Box<dyn Fn(bool)>;

props! {
    TooltipRootProps {
        delay_duration: i32,

        default_open: bool,

        open: bool,

        on_open_change: EventHandler<bool>,

        on_update_open: EventHandler<bool>,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
