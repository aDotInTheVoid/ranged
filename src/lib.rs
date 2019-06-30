//! Note: this crate realy wants `const fn`s in traits.  

mod impls;
pub use impls::*;


mod helpers;
use std::cmp;

// mod helpers;

/// Anything that behaves like a number
pub trait Num: Copy + num_traits::NumAssign + std::cmp::Ord {}

/// Hack to impl `T` Everywhere
impl<T> Num for T where T: Copy + num_traits::NumAssign + std::cmp::Ord {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ranged<T: Num> {
    min: T,
    max: T,
}

impl<T: Num> Ranged<T> {
    pub fn new(a: T, b: T) -> Ranged<T> {
        Ranged {
            min: cmp::min(a, b),
            max: cmp::min(a, b),
        }
    }
}

// The goal is to uncoment these lines and have it work
// We cant impl Num because Num Needs Ord for comparisons to see which args
// will be min and max. However we cant impl ord for range because overlaping
// ranges are unclear
//impl<T: Num> num_traits::NumAssign for Ranged<T>{
//
//}

