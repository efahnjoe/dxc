use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::IntoAttributeValue;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

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

impl IntoAttributeValue for Loading {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}
