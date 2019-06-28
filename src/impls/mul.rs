use crate::{Num, Ranged};
use std::ops::*;

impl<T: Num> Mul for Ranged<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        // TODO: Impl
        Ranged {
            min: T::zero(),
            max: T::zero(),
        }
    }
}
