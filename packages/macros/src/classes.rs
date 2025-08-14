#[macro_export]
macro_rules! classes {
    () => {
        String::new()
    };

    // 接收任意数量的 impl AsRef<str> 表达式，用空格拼接
    ($($class:expr),* $(,)?) => {{
        let mut classes = Vec::new();
        $(
            let s: String = $class.to_string();
            if !s.is_empty() {
                classes.push(s);
            }
        )*
        classes.join(" ")
    }};
}
