use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

pub fn app() -> Element {
    let img_list = vec![
        String::from("https://picsum.photos/200/300"),
        String::from("https://picsum.photos/200/300"),
        String::from("https://picsum.photos/200/300"),
        String::from("https://picsum.photos/200/300"),
    ];

    rsx!(
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            style: "width: 200px; height: 200px;",
            DxcImage {
                src: "https://picsum.photos/200/300",
                crossorigin: "anonymous",
                preview_src_list: img_list
            }
        }
    )
}
