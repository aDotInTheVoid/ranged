use ranged;
use ranged::{Num, Ranged};

fn r<T: ranged::Num>(a: T, b: T) -> Ranged<T> {
    Ranged::new(a, b)
}

#[test]
fn addition() {
    let r35 = r(3_i32, 5);
    let r57 = r(5_i32, 7);

    assert_eq!(r35 + r35, r(6, 10));
    assert_eq!(r57 + r57, r(10, 14));
    assert_eq!(r35 + r57, r(8, 12));
    assert_eq!(r57 + r35, r(8, 12));
}
