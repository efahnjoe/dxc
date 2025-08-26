use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::IntoAttributeValue;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum Direction {
    #[serde(rename = "vertical")]
    #[strum(serialize = "vertical")]
    Vertical,

    #[serde(rename = "horizontal")]
    #[strum(serialize = "horizontal")]
    Horizontal,
}

impl IntoAttributeValue for Direction {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}
