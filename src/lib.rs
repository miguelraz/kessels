fn main() {}

pub mod kessels {
    pub fn work1<T: From<i8> + std::ops::AddAssign<T>>(_i: T) -> T {
        let mut _sum = T::from(0);
        for iter in 1..11 {
            _sum += T::from(iter);
        }
        _sum
    }
}
//use num::One;
//use std::iter::Sum;
//use std::ops::Range;

//pub fn work2<T>() -> T
//where
//    T: One + Sum + Clone,
//    Range<T>: Iterator<Item = T>,
//{
//    let eleven = std::iter::repeat(T::one()).take(11).sum();
//    (T::one()..eleven).sum()
//}

#[test]
fn it_works() {
    assert_eq!(work::<i32>(), 55);
    assert_eq!(work::<u32>(), 55);
}
