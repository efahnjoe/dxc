#[cfg(test)]
mod class_test {
    use dxc_macros::classes;

    #[test]
    fn class_test() {
        let input = classes! {
            "my-class",
            if true { "my-class" } else { "my-other-class" },
            format!("my-class-{}", 1)
        };

        assert_eq!(input, "my-class my-class my-class-1".to_string())
    }
}
