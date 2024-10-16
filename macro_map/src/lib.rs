#[macro_export]
macro_rules! hash_map {
    // Handle the empty case
    () => {std::collections::HashMap::new()};
    // Handle trailing commas or multiple key-value pairs
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}
