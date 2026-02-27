use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Resize {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,

    #[serde(rename = "both")]
    #[strum(serialize = "both")]
    Both,

    #[serde(rename = "horizontal")]
    #[strum(serialize = "horizontal")]
    Horizontal,

    #[serde(rename = "vertical")]
    #[strum(serialize = "vertical")]
    Vertical,
}

pub use crate::types::common::size::Size;
