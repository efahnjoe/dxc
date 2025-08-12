use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

pub fn app() -> Element {
    rsx!(
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            style: "width: 200px; height: 200px;",
            DxcImage {
                src: "https://fuss10.elemecdn.com/a/3f/3302e58f9a181d2509f3dc0fa68b0jpeg.jpeg"
            }
        }
    )
}
