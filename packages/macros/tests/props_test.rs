use dioxus::prelude::*;
use dxc_macros::props;

props! {
    TestProps {
        type_: String,
        disabled: bool,
    }
}

#[cfg(test)]
mod props_tests {
    use super::TestProps;

    #[test]
    fn props_test() {
        let props = TestProps {
            type_: Some("button".to_string()),
            disabled: Some(true),
        };

        assert_eq!(props.type_.as_deref(), Some("button"));
        assert!(props.disabled.unwrap_or(false));
    }
}
