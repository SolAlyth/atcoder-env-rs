use crate::mylib::util::macros::impl_for;

pub trait ChangeMinMax: Sized + PartialOrd + Copy {
    fn chmax(&mut self, value: Self) -> bool { let tmp = *self < value; if tmp { *self = value; } tmp }
    fn chmin(&mut self, value: Self) -> bool { let tmp = value < *self; if tmp { *self = value; } tmp }
}

impl_for!(ChangeMinMax; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait WrappingAddSignedForPair {
    fn wrapping_add_signed(self, rhs: (isize, isize)) -> Self;
}

impl WrappingAddSignedForPair for (usize, usize) {
    fn wrapping_add_signed(self, rhs: (isize, isize)) -> Self { (self.0.wrapping_add_signed(rhs.0), self.1.wrapping_add_signed(rhs.1)) }
}
