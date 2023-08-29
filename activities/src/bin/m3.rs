// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.


macro_rules! hashmap {
    (
        $( $key:expr => $value:expr ),+
        $(,)?
    ) => {{
        let mut h = ::std::collections::HashMap::new();
        $(
            h.insert($key, $value);
        )+
        h
    }};
}
fn main() {
    let hashmap = hashmap!{
        1 => "a",
        2 => "b",
        3 => "c",
    };
    dbg!(hashmap);
}
