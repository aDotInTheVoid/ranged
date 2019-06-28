use crate::{Range, Num};
use std::ops::*;

/// Trivial implementation
/// 
/// # Implementation
/// [![Graph z=x+y]()]
impl<T: Num> Add for Range<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Range {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}