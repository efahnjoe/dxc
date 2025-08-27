use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

pub fn app() -> Element {
    let img_list = vec![
        String::from("https://picsum.photos/400/250?random=1"),
        String::from("https://picsum.photos/400/250?random=2"),
        String::from("https://picsum.photos/400/250?random=3"),
        String::from("https://picsum.photos/400/250?random=4"),
    ];

    rsx!(
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            style: "width: 200px; height: 200px;",
            DxcImage {
                src: "https://picsum.photos/400/250",
                crossorigin: image::Crossorigin::Anonymous,
                preview_src_list: img_list.clone(),
                show_progress: true,
            }
        }
    )
}
