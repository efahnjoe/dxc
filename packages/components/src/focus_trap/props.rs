use dioxus::prelude::*;
use dxc_macros::{props,PropsDefault};

props! {
    FocusTrapProps {
        loop_: bool,
        trapped: bool,

        #[props_default(skip)]
        focus_trap_element: Element,

        #[props_default(skip)]
        focus_start_element: Element,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}