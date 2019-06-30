use ranged;

#[allow(unused_imports)]
use ranged::{Num, Ranged};

fn r<T: ranged::Num>(a: T, b: T) -> Ranged<T> {
    Ranged::new(a, b)
}

mod ranged_ranged {
    use super::*;

    #[test]
    fn plus_plus() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);

        assert_eq!(r35 + r35, r(6, 10));
        assert_eq!(r57 + r57, r(10, 14));
        assert_eq!(r35 + r57, r(8, 12));
        assert_eq!(r57 + r35, r(8, 12));
    }

    #[test]
    fn identity() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);
        let id = r(0, 0);

        assert_eq!(r35 + id, r35);
        assert_eq!(r35 + id, r35);

        assert_eq!(r57 + id, r57);
        assert_eq!(r57 + id, r57);

        assert_eq!(id + id, id);
    }
}

mod ranged_const {
    use super::*;

    #[test]
    fn plus_plus() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);
        assert_eq!(r35 + 7, r(10, 12));
        assert_eq!(r35 + 1, r(4, 6));
        assert_eq!(r57 + 7, r(12, 14));
        assert_eq!(r57 + 1, r(6, 8));
    }

    #[test]
    fn identity() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);
        let id = 0;

        assert_eq!(r35 + id, r35);
        assert_eq!(r35 + id, r35);

        assert_eq!(r57 + id, r57);
        assert_eq!(r57 + id, r57);

        assert_eq!(id + id, id);
    }
}