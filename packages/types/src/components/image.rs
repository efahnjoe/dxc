use dioxus::dioxus_core::{AttributeValue, IntoAttributeValue};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub use crate::common::crossorigin::Crossorigin;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Fit {
    #[serde(rename = "contain")]
    #[strum(serialize = "contain")]
    Contain,

    #[serde(rename = "cover")]
    #[strum(serialize = "cover")]
    Cover,

    #[serde(rename = "fill")]
    #[strum(serialize = "fill")]
    Fill,

    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,

    #[serde(rename = "scale-down")]
    #[strum(serialize = "scale-down")]
    ScaleDown,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Loading {
    #[serde(rename = "eager")]
    #[strum(serialize = "eager")]
    Eager,
    #[serde(rename = "lazy")]
    #[strum(serialize = "lazy")]
    Lazy,
}

impl IntoAttributeValue for Fit {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}

impl IntoAttributeValue for Loading {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}
