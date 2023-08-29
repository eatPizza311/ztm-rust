
macro_rules! myvec {
    (
        $( $element:expr ),+
        $(,)?
    ) => {{
        let mut v = Vec::new();
        $(
            v.push($element);
        )+
        v
    }};
}

fn main() {
    let v1 = myvec![1, 2, 3, 4];
    let v2 = {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v
    };
    assert_eq!(v1, v2)
}
