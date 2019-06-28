use crate::{Num, Ranged};
use std::ops::*;

/// Trivial implementation
///
/// # Implementation
/// [![Graph z=x+y]()]
impl<T: Num> Add for Ranged<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Ranged {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}

impl<T: Num> Add<T> for Ranged<T> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Ranged {
            min: self.min + other,
            max: self.max + other,
        }
    }
}
