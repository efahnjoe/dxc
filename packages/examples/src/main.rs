mod button;
mod icon;
mod image;
mod input;
mod link;

use crate::button::Button;
use crate::icon::Icon;
use crate::image::Image;
use crate::input::Input;
use crate::link::Link;
use dioxus::prelude::*;
use dxc::prelude::*;

pub fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link{rel: "stylesheet", href: DXC_THEMES}

        Button {}
        Icon {}
        Image {}
        Input {}
        Link {}
    }
}
