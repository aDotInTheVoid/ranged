/// Trivial implementation
impl<T: Num> Add for Range<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Range {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}