impl<T: Num> Mul for Range<T>{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Range {
            min: 0,
            max: 0,
        }
    }
}