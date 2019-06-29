use crate::Num;
use std::cmp::{max, min};

pub(crate) fn max4<T: Num>(a: T, b: T, c: T, d: T) -> T {
    max(max(a, b), max(c, d))
}

pub(crate) fn min4<T: Num>(a: T, b: T, c: T, d: T) -> T {
    min(min(a, b), min(c, d))
}
