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

        assert_eq!(r35 - r35, r(-2, 2));
        assert_eq!(r57 - r57, r(-2, 2));
        assert_eq!(r35 - r57, r(-4, 0));
        assert_eq!(r57 - r35, r(0, 4));
    }

    #[test]
    fn identity() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);
        let id = r(0, 0);

        assert_eq!(r35 - id, r35);
        assert_eq!(r35 - id, r35);

        assert_eq!(r57 - id, r57);
        assert_eq!(r57 - id, r57);

        assert_eq!(id - id, id);
    }
}

mod ranged_const {
    use super::*;

    #[test]
    fn plus_plus() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);
        assert_eq!(r35 - 7, r(-4, -2));
        assert_eq!(r35 - 1, r(2, 4));
        assert_eq!(r57 - 7, r(-2, 0));
        assert_eq!(r57 - 1, r(4, 6));
    }

    #[test]
    fn identity() {
        let r35 = r(3_i32, 5);
        let r57 = r(5_i32, 7);
        let id = 0;

        assert_eq!(r35 - id, r35);
        assert_eq!(r35 - id, r35);

        assert_eq!(r57 - id, r57);
        assert_eq!(r57 - id, r57);

        assert_eq!(id - id, id);
    }
}
