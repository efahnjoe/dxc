use dioxus::prelude::*;
use dxc::prelude::*;

#[component]
pub fn Image() -> Element {
    let img_list = vec![
        String::from("https://picsum.photos/400/250?random=1"),
        String::from("https://picsum.photos/400/250?random=2"),
        String::from("https://picsum.photos/400/250?random=3"),
        String::from("https://picsum.photos/400/250?random=4"),
    ];

    rsx!(
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
