use crate::{Num, Ranged};
use std::ops::*;

/// Trivial Implementation
impl<T: Num> Sub for Ranged<T> {
    type Output = Ranged<T>;
    fn sub(self, other: Self) -> Self::Output {
        Ranged {
            min: self.min - other.max,
            max: self.max + other.min,
        }
    }
}
