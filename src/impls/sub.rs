use crate::{Num, Ranged};
use std::ops::*;

// Allows `Ranged<T> - Ranged<T> = Ranged<T>`
impl<T: Num> Sub for Ranged<T> {
    type Output = Ranged<T>;
    fn sub(self, other: Self) -> Self::Output {
        Ranged {
            min: self.min - other.max,
            max: self.max - other.min,
        }
    }
}

impl<T: Num> Sub<T> for Ranged<T> {
    type Output = Self;
    fn sub(self, other: T) -> Self::Output {
        Ranged {
            min: self.min - other,
            max: self.max - other,
        }
    }
}

// We cant do this because orphan rules
// impl<T: Num> Sub<Ranged<T>> for T {
// }
