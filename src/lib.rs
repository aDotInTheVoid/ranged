
pub trait Num: Copy + num_traits::NumAssign{}

#[derive(Clone, Copy)]
pub struct Range<T: Num>{
    min: T,
    max: T,
}

impl<T: Num> Num for Range<T>{

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
