use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum Type {
    #[serde(rename = "primary")]
    #[strum(serialize = "primary")]
    Primary,

    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,

    #[serde(rename = "warning")]
    #[strum(serialize = "warning")]
    Warning,

    #[serde(rename = "danger")]
    #[strum(serialize = "danger")]
    Danger,

    #[serde(rename = "info")]
    #[strum(serialize = "info")]
    Info,
}

pub use crate::types::common::size::Size;
