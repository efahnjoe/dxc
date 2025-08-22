use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::IntoAttributeValue;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Crossorigin {
    #[serde(rename = "anonymous")]
    #[strum(serialize = "anonymous")]
    Anonymous,

    #[serde(rename = "use-credentials")]
    #[strum(serialize = "use-credentials")]
    UseCredentials,
}

impl IntoAttributeValue for Crossorigin {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}
