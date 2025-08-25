use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::IntoAttributeValue;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum LinkType {
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

impl IntoAttributeValue for LinkType {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}
