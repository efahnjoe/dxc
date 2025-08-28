use dioxus::prelude::*;
use dxc::prelude::*;

#[component]
pub fn Button() -> Element {
    rsx! {
        div {
            div {
                DxcButton {
                    "Default"
                },
                DxcButton {
                    type_: button::Type::Primary,
                    "Primary"
                },
                DxcButton {
                    type_: button::Type::Success,
                    "Seccess"
                },
                DxcButton {
                    type_: button::Type::Info,
                    "Info"
                },
                DxcButton {
                    type_: button::Type::Warning,
                    "Warning"
                },
                DxcButton {
                    type_: button::Type::Danger,
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
                    type_: button::Type::Primary,
                    "Primary"
                },
                DxcButton {
                    plain: true,
                    type_: button::Type::Success,
                    "Seccess"
                },
                DxcButton {
                    plain: true,
                    type_: button::Type::Info,
                    "Info"
                },
                DxcButton {
                    plain: true,
                    type_: button::Type::Warning,
                    "Warning"
                },
                DxcButton {
                    plain: true,
                    type_: button::Type::Danger,
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
                    type_: button::Type::Primary,
                    "Primary"
                },
                DxcButton {
                    round: true,
                    type_: button::Type::Success,
                    "Seccess"
                },
                DxcButton {
                    round: true,
                    type_: button::Type::Info,
                    "Info"
                },
                DxcButton {
                    round: true,
                    type_: button::Type::Warning,
                    "Warning"
                },
                DxcButton {
                    round: true,
                    type_: button::Type::Danger,
                    "Danger"
                },
            }

            div {
                DxcButton {
                    icon: "Search",
                    circle: true,
                },
                DxcButton {
                    type_: button::Type::Primary,
                    icon: "Edit",
                    circle: true,
                },
                DxcButton {
                    type_: button::Type::Success,
                    icon: "Check",
                    circle: true,
                },
                DxcButton {
                    type_: button::Type::Info,
                    icon: "Message",
                    circle: true,
                },
                DxcButton {
                    type_: button::Type::Warning,
                    icon: "Star",
                    circle: true,
                },
                DxcButton {
                    type_: button::Type::Danger,
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
                    type_: button::Type::Primary,
                    "Primary"
                },
                DxcButton {
                    disabled: true,
                    type_: button::Type::Success,
                    "Seccess"
                },
                DxcButton {
                    disabled: true,
                    type_: button::Type::Info,
                    "Info"
                },
                DxcButton {
                    disabled: true,
                    type_: button::Type::Warning,
                    "Warning"
                },
                DxcButton {
                    disabled: true,
                    type_: button::Type::Danger,
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
                    type_: button::Type::Primary,
                    "Primary"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: button::Type::Success,
                    "Seccess"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: button::Type::Info,
                    "Info"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: button::Type::Warning,
                    "Warning"
                },
                DxcButton {
                    disabled: true,
                    plain: true,
                    type_: button::Type::Danger,
                    "Danger"
                },
            }
        }
    }
}
