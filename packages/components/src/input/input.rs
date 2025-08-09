// use crate::emitter::{Emitter, EventData};
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;

#[component]
pub fn DxcInput(children: Element) -> Element {
    let ns = UseNamespace::new("input", None);

    rsx! {
        div {
            class:ns.bm_("group", "append"),
            onmouseenter:move |event| handle_mouse_enter(event),
            onmouseleave:move |event| handle_mouse_leave(event),
            input {}
        }
    }
}

// let hovering = use_state(|| false);

struct InputState {
    hovering: bool,
}

fn handle_mouse_enter(evt: MouseEvent) -> () {
    InputState { hovering: true };
    // Emitter::new(evt).emit("input:hover", ());
}

fn handle_mouse_leave(evt: MouseEvent) -> () {
    // InputState { hovering: false };
}
