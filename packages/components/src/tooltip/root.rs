use super::props::TooltipRootProps;
use dioxus::prelude::*;
use dxc_hooks::UseNamespace;
use dxc_types::namespace::Block;

#[component]
pub fn DxcTooltipRoot(props: TooltipRootProps) -> Element {
    let on_open_change = props.on_open_change().clone();

    let mut open = use_signal(|| if props.open() { true } else { props.open() });
    let is_open_delayed = use_signal(|| props.delay_duration() > 0);

    let mut on_normal_open = move || {
        open.set(true);
    };

    let on_delay_open = move || {
        if is_open_delayed() {
            // on_delayed_open();
        } else {
            on_normal_open();
        }
    };

    let on_open = move || {};

    let on_close = move || {
        open.set(false);
    };

    let on_change = move |open: bool| {
        if open {
            on_open()
        }

        on_open_change(open);
    };

    let ns = UseNamespace::new(Block::TooltipV2, None);

    rsx! {
        {props.children}
    }
}
