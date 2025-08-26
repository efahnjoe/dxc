use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};
use dxc_types::components::container::Direction;

props! {
    ContainerProps {
        class: String,

        #[props_default(value = Direction::Horizontal)]
        direction: Direction,

        style: String,

        #[props_default(skip)]
        children: Element,
    },
    derive(PropsDefault)
}
