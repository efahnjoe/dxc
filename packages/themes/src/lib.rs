#![deprecated(note = "This crate is deprecated. Please switch to `dxc` for future updates.")]

// include!(concat!(env!("OUT_DIR"), "/generated_themes.rs"));

use dioxus::prelude::*;

pub const DXC_THEMES: Asset = asset!("assets/css/index.css");
