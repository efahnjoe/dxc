use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};
use dxc_types::Direction;

props! {
    ContainerProps {
        class: String,

        #[props_default(value = Direction::Horizontal)]
        direction: Direction,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
