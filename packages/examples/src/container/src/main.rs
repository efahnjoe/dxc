use crate::layout_ahmf::LayoutAhmf;
use dioxus::prelude::*;
use dxc::prelude::*;
mod layout_ahmf;

const COMMON_LAYOUT_SCSS: Asset = asset!("examples/container/src/common-layout.scss");

pub fn main() {
    dioxus::launch(app);
}

pub fn app() -> Element {
    rsx!(
      document::Link{rel: "stylesheet", href: COMMON_LAYOUT_SCSS}
      document::Link{rel: "stylesheet", href: DXC_THEMES}

      div {
        div {
          LayoutAhmf{}
        }
       }
    )
}
