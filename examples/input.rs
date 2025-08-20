use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

#[component]
pub fn app() -> Element {
    let value = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: DXC_THEMES}

        div {
            style: "width: 200px",
            "Basic usage"
            DxcInput {value:value,id: "input-id",}

            "Disabled"
            DxcInput {
                disabled: true,
            }

            "Clearable"
            DxcInput {
                value:value,
                clearable: true,
            }

            "Formatter"
            DxcInput {
                // formatter: |value| {
                //     value.to_string()
                // }
            }

            "Password box"
            DxcInput {
                value:value,
                show_password: true,
            }

            "Input with icon"
            div {
                p {"Suffix icon"}
                DxcInput {
                    value:value,
                    suffix_icon: "Calendar".to_string(),
                }
                p {"Prefix icon"}
                DxcInput {
                    value:value,
                    prefix_icon: "Search".to_string(),
                }
            }
        }
    }
}
