#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EventCodeType {
    pub tab: &'static str,
    pub enter: &'static str,
    pub space: &'static str,
    pub left: &'static str,
    pub up: &'static str,
    pub right: &'static str,
    pub down: &'static str,
    pub esc: &'static str,
    pub delete: &'static str,
    pub backspace: &'static str,
    pub numpad_enter: &'static str,
    pub page_up: &'static str,
    pub page_down: &'static str,
    pub home: &'static str,
    pub end: &'static str,
}

pub static EVENT_CODE: EventCodeType = EventCodeType {
    tab: "Tab",
    enter: "Enter",
    space: "Space",
    left: "ArrowLeft",   // 37
    up: "ArrowUp",       // 38
    right: "ArrowRight", // 39
    down: "ArrowDown",   // 40
    esc: "Escape",
    delete: "Delete",
    backspace: "Backspace",
    numpad_enter: "NumpadEnter",
    page_up: "PageUp",
    page_down: "PageDown",
    home: "Home",
    end: "End",
};
