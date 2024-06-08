use crate::mylib::util::macros::impl_for;

pub trait ChangeMinMax: Sized + PartialOrd + Copy {
    fn chmax(&mut self, value: Self) -> bool { let tmp = *self < value; if tmp { *self = value; } tmp }
    fn chmin(&mut self, value: Self) -> bool { let tmp = value < *self; if tmp { *self = value; } tmp }
}

impl_for!(ChangeMinMax; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait WrappingAddSignedForPair {
    const LRUD: [(isize, isize); 4] = [(0, -1), (0, 1), (0, -1), (0, 1)];
    fn wrapping_add_signed(self, rhs: (isize, isize)) -> Self;
}

impl WrappingAddSignedForPair for (usize, usize) {
    fn wrapping_add_signed(self, rhs: (isize, isize)) -> Self { (self.0.wrapping_add_signed(rhs.0), self.1.wrapping_add_signed(rhs.1)) }
}



pub trait CharUtil: Clone {
    const lower: [Self; 26];
    const upper: [Self; 26];
    
    fn lower_to_us(self) -> usize;
    fn upper_to_us(self) -> usize;
    
    fn flip(self) -> Self;
}

impl CharUtil for char {
    const lower: [char; 26] = {
        let (mut out, mut i) = (['_'; 26], 0);
        while i < 26 { out[i] = (i+97) as u8 as char; i += 1; }
        out
    };
    
    const upper: [char; 26] = {
        let (mut out, mut i) = (['_'; 26], 0);
        while i < 26 { out[i] = (i+65) as u8 as char; i += 1; }
        out
    };
    
    fn lower_to_us(self) -> usize { self as usize - 97 }
    fn upper_to_us(self) -> usize { self as usize - 65 }
    
    fn flip(self) -> Self { (self as u8 ^ 32) as char }
}
