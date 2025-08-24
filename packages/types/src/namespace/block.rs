use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum Block {
    #[serde(rename = "button")]
    #[strum(serialize = "button")]
    Button,
}
