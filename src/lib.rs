use std::ops::*;

pub trait Num: Copy + num_traits::NumAssign {}

#[derive(Clone, Copy)]
pub struct Range<T: Num> {
    min: T,
    max: T,
}

//impl<T: Num> Num for Range<T>{
//
//}

impl<T: Num> Add for Range<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Range {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
