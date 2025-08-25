use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum Block {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,

    #[serde(rename = "button")]
    #[strum(serialize = "button")]
    Button,

    #[serde(rename = "container")]
    #[strum(serialize = "container")]
    Container,

    #[serde(rename = "aside")]
    #[strum(serialize = "aside")]
    Aside,

    #[serde(rename = "footer")]
    #[strum(serialize = "footer")]
    Footer,

    #[serde(rename = "header")]
    #[strum(serialize = "header")]
    Header,

    #[serde(rename = "main")]
    #[strum(serialize = "main")]
    Main,

    #[serde(rename = "focus-trap")]
    #[strum(serialize = "focus-trap")]
    FocusTrap,

    #[serde(rename = "icon")]
    #[strum(serialize = "icon")]
    Icon,

    #[serde(rename = "image")]
    #[strum(serialize = "image")]
    Image,

    #[serde(rename = "image-viewer")]
    #[strum(serialize = "image-viewer")]
    ImageViewer,

    #[serde(rename = "input")]
    #[strum(serialize = "input")]
    Input,

    #[serde(rename = "link")]
    #[strum(serialize = "link")]
    Link,

    #[serde(rename = "menu")]
    #[strum(serialize = "menu")]
    Menu,

    #[serde(rename = "teleport")]
    #[strum(serialize = "teleport")]
    Teleport,

    #[serde(rename = "textarea")]
    #[strum(serialize = "textarea")]
    Textarea,

    #[serde(rename = "transition")]
    #[strum(serialize = "transition")]
    Transition,
}
