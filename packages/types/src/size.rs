use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::IntoAttributeValue;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Size {
    #[serde(rename = "large")]
    #[strum(serialize = "large")]
    Large,

    #[serde(rename = "default")]
    #[strum(serialize = "default")]
    Default,

    #[serde(rename = "small")]
    #[strum(serialize = "small")]
    Small,
}

impl IntoAttributeValue for Size {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}
