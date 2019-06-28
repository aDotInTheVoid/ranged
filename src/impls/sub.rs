use crate::{Range, Num};
use std::ops::*;

/// Trivial Implementation
impl<T:Num> Sub for Range<T>{
    type Output = Range<T>;
    fn sub(self, other: Self) -> Self::Output {
        Range {
            min: self.min - other.max,
            max: self.max + other.min,
        }
    }
}