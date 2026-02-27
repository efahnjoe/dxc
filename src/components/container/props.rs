use crate::types::components::container::Direction;
use dioxus::prelude::*;
use dxc_macros::{PropsDefault, props};

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
