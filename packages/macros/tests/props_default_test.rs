use dioxus::prelude::*;
use dxc_macros::{props, PropsDefault};

props! {
    TestProps {
        #[props_default(value = "button")]
        type_: String,
        disabled: bool,
        #[props_default(skip)]
        skipped_field: String,
    },
    derive(PropsDefault)
}

#[cfg(test)]
mod props_default_tests {
    use super::*;

    /// When the field is not set, the getter returns the default value.
    #[test]
    fn default_values_are_used_when_none() {
        let props = TestProps::default();

        assert_eq!(props.type_(), "button");
        assert_eq!(props.disabled(), false);
    }

    /// The custom value was returned correctly.
    #[test]
    fn custom_values_are_respected() {
        let props = TestProps {
            type_: Some("submit".to_string()),
            disabled: Some(true),
            ..TestProps::default()
        };

        assert_eq!(props.type_(), "submit");
        assert_eq!(props.disabled(), true);
    }

    /// Partial field settings
    #[test]
    fn partial_initialization_works() {
        let props = TestProps {
            type_: None,
            disabled: Some(true),
            ..TestProps::default()
        };

        assert_eq!(props.type_(), "button");
        assert_eq!(props.disabled(), true);
    }

    /// `disabled` fallback to false
    #[test]
    fn disabled_falls_back_to_false() {
        let props = TestProps {
            disabled: None,
            ..TestProps::default()
        };

        assert_eq!(props.disabled(), false);
    }

    /// Custom values are set directly through fields.
    #[test]
    fn custom_values_via_field_work() {
        let props = TestProps {
            type_: Some("submit".to_string()),
            disabled: Some(true),
            skipped_field: Some("manual".to_string()),
            ..TestProps::default()
        };

        assert_eq!(props.type_(), "submit");
        assert_eq!(props.disabled(), true);
        assert_eq!(props.skipped_field.as_deref(), Some("manual"));
    }

    /// `skipped_field` is None when not set.
    #[test]
    fn skipped_field_is_none_by_default() {
        let props = TestProps::default();
        assert_eq!(props.skipped_field, None);
    }

    /// Clone normally
    #[test]
    fn props_can_be_cloned() {
        let props = TestProps {
            type_: Some("clone_test".to_string()),
            disabled: Some(false),
            ..TestProps::default()
        };

        let cloned = props.clone();
        assert_eq!(props.type_(), cloned.type_());
        assert_eq!(props.disabled(), cloned.disabled());
    }
}
