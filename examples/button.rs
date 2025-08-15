use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(app);
}

#[component]
pub fn app() -> Element {
    rsx! {
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        div {
            div {
                DxcButton {
                    "Default"
                },
                DxcButton {
                    type_: "primary",
                    "Primary"
                },
                DxcButton {
                    type_: "success",
                    "Seccess"
                },
                DxcButton {
                    type_: "info",
                    "Info"
                },
                DxcButton {
                    type_: "warning",
                    "Warning"
                },
                DxcButton {
                    type_: "danger",
                    "Danger"
                },
            }

            div {
                DxcButton {
                    plain: true,
                    "Plain"
                },
                DxcButton {
                    plain: true,
                    type_: "primary",
                    "Primary"
                },
                DxcButton {
                    plain: true,
                    type_: "success",
                    "Seccess"
                },
                DxcButton {
                    plain: true,
                    type_: "info",
                    "Info"
                },
                DxcButton {
                    plain: true,
                    type_: "warning",
                    "Warning"
                },
                DxcButton {
                    plain: true,
                    type_: "danger",
                    "Danger"
                },
            }

            div {
                DxcButton {
                    round: true,
                    "Default"
                },
                DxcButton {
                    round: true,
                    type_: "primary",
                    "Primary"
                },
                DxcButton {
                    round: true,
                    type_: "success",
                    "Seccess"
                },
                DxcButton {
                    round: true,
                    type_: "info",
                    "Info"
                },
                DxcButton {
                    round: true,
                    type_: "warning",
                    "Warning"
                },
                DxcButton {
                    round: true,
                    type_: "danger",
                    "Danger"
                },
            }

            div {
                DxcButton {
                    icon: "Search",
                    circle: true,
                },
                DxcButton {
                    type_: "primary",
                    icon: "Edit",
                    circle: true,
                },
                DxcButton {
                    type_: "success",
                    icon: "Check",
                    circle: true,
                },
                DxcButton {
                    type_: "info",
                    icon: "Message",
                    circle: true,
                },
                DxcButton {
                    type_: "warning",
                    icon: "Star",
                    circle: true,
                },
                DxcButton {
                    type_: "danger",
                    icon: "Delete",
                    circle: true,
                },
            }
        }

        div {
            div {
                DxcButton {
                    disabled: true,
                    "Default"
                },
                DxcButton {
                    disabled: true,
                    type_: "primary",
                    "Primary"
                },
                DxcButton {
                    disabled: true,
                    type_: "success",
                    "Seccess"
                },
                DxcButton {
                    disabled: true,
                    type_: "info",
                    "Info"
                },
                DxcButton {
                    disabled: true,
                    type_: "warning",
                    "Warning"
                },
                DxcButton {
                    disabled: true,
                    type_: "danger",
                    "Danger"
                },
            }

            div {
                DxcButton {
                    disabled: true,
                    plain: true,
                    "Plain"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: "primary",
                    "Primary"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: "success",
                    "Seccess"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: "info",
                    "Info"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: "warning",
                    "Warning"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: "danger",
                    "Danger"
                },
            }
        }
    }
}
