macro_rules! html {
    //base case
    ($w:expr,) => {}; // this is the "&mut data," part

    // recursive
    // for the most inner case like "Demo title"
    ($w:expr, $e:tt) => (write!($w, "{}", $e));
    // for the case with [] in it
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)* ) => {{
        write!($w, "<{}>", stringify!($tag));
        html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        html!($w, $($rest)*);
    }};
}

#[allow(unused_must_use)]
fn main() {
    use std::fmt::Write;
    let mut data = String::new();
    html!(&mut data,
    html[
        head[ title["Demo title"] ]
        body[
            h1["Sample"]
            p["This is a macro demo"]
        ]
    ]);
    dbg!(data);
}
