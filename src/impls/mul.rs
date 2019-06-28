use crate::{Range, Num};
use std::ops::*;

impl<T: Num> Mul for Range<T>{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        // TODO: Impl
        Range {
            min: T::zero(),
            max: T::zero(),
        }
    }
}