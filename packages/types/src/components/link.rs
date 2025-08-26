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

    #[serde(rename = "default")]
    #[strum(serialize = "default")]
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum Underline {
    #[serde(rename = "always")]
    #[strum(serialize = "always")]
    Always,

    #[serde(rename = "never")]
    #[strum(serialize = "never")]
    Never,

    #[serde(rename = "hover")]
    #[strum(serialize = "hover")]
    Hover,

    #[serde(rename = "true")]
    #[strum(serialize = "true")]
    True,

    #[serde(rename = "false")]
    #[strum(serialize = "false")]
    False,
}
