//! Note: this crate realy wants `const fn`s in traits.  

mod impls;
pub use impls::*;

// mod helpers;

/// Anything that behaves like a number
pub trait Num: Copy + num_traits::NumAssign + std::cmp::Ord {}

#[derive(Clone, Copy)]
pub struct Ranged<T: Num> {
    min: T,
    max: T,
}

// The goal is to uncoment these lines and have it work
// We cant impl Num because Num Needs Ord for comparisons to see which args
// will be min and max. However we cant impl ord for range because overlaping
// ranges are unclear
//impl<T: Num> num_traits::NumAssign for Ranged<T>{
//
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
