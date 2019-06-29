use crate::{Num, Ranged};
use std::ops::*;

use crate::helpers;
use helpers::{max4, min4};

/// `Ranged<T> * Ranged<T> = Ranged<T>`
impl<T: Num> Mul for Ranged<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        // Because I can't be bothered to work out a nice way of doing this, we will bodge it
        let x1 = self.min * other.min;
        let x2 = self.min * other.max;
        let x3 = self.max * other.min;
        let x4 = self.max * other.max;
        Ranged {
            min: min4(x1, x2, x3, x4),
            max: max4(x1, x2, x3, x4),
        }
    }
}
