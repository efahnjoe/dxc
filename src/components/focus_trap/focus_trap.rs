use super::props::FocusTrapProps;
use dioxus::prelude::*;

#[component]
pub fn DxcFocusTrap(props: FocusTrapProps) -> Element {
    let loop_ = use_signal(|| props.loop_.unwrap_or(false));
    let trapped = use_signal(|| props.trapped.unwrap_or(false));

    let mut alt_key = use_signal(|| false);
    let mut ctrl_key = use_signal(|| false);
    let mut shift_key = use_signal(|| false);
    let mut meta_key = use_signal(|| false);
    let mut esc_pressed = use_signal(|| false);
    let mut tab_pressed = use_signal(|| false);

    let on_keydown = move |event: Event<KeyboardEvent>| {
        if !loop_() && !trapped() {
            return;
        };
        let key_board = event.data;
        let code = key_board.code();

        let is_tabbing = use_memo(move || {
            code == Code::Tab && !alt_key() && !ctrl_key() && !meta_key() && !shift_key()
        });

        if is_tabbing() {};
    };

    let mut has_focus = use_signal(|| false);

    rsx! {
        div {
            style: "display: contents;",
            tabindex: "0",
            onkeydown: move |evt| {
                alt_key.set(evt.modifiers().alt());
                ctrl_key.set(evt.modifiers().ctrl());
                shift_key.set(evt.modifiers().shift());
                meta_key.set(evt.modifiers().meta());

                match evt.key() {
                    Key::Escape => {
                        esc_pressed.set(true);
                    }
                    Key::Tab => {
                        tab_pressed.set(true);
                        evt.prevent_default();
                    }
                    _ => {}
                }
            },
            onkeyup: move |evt: KeyboardEvent| {
                if !evt.modifiers().alt() { alt_key.set(false); }
                if !evt.modifiers().ctrl() { ctrl_key.set(false); }
                if !evt.modifiers().shift() { shift_key.set(false); }
                if !evt.modifiers().meta() { meta_key.set(false); }

                match evt.key() {
                    Key::Escape => {
                        esc_pressed.set(false);
                    }
                    Key::Tab => {
                        tab_pressed.set(false);
                        evt.prevent_default();
                    }
                    _ => {}
                }
            },
            onfocusin: move |_| {
                has_focus.set(true);
            },
            onfocusout: move |_| {
                has_focus.set(false);
            },
            {props.children}
        }
    }
}
