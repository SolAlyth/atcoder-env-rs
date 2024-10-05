use std::ops::{RangeBounds, Bound::*};

pub trait RectUtil: Sized + Copy {
    type Rhs: Copy;
    const LRUD: [Self::Rhs; 4];
    
    fn wrapping_add_signed(self, rhs: Self::Rhs) -> Self;
    fn apply_lrud(self) -> [Self; 4];
}

impl RectUtil for (usize, usize) {
    type Rhs = (isize, isize);
    const LRUD: [Self::Rhs; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    
    fn wrapping_add_signed(self, rhs: Self::Rhs) -> Self { (self.0.wrapping_add_signed(rhs.0), self.1.wrapping_add_signed(rhs.1)) }
    fn apply_lrud(self) -> [Self; 4] { Self::LRUD.map(|d| self.wrapping_add_signed(d)) }
}



pub trait CharUtil: Clone {
    const lower: [Self; 26];
    const upper: [Self; 26];
    
    fn lower_to_us(self) -> usize;
    fn upper_to_us(self) -> usize;
    
    fn flip(self) -> Self;
    
    fn as_lrud(self) -> usize;
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
    
    fn lower_to_us(self) -> usize { debug_assert!('a' <= self && self <= 'z'); self as usize - 97 }
    fn upper_to_us(self) -> usize { debug_assert!('A' <= self && self <= 'Z'); self as usize - 65 }
    
    fn flip(self) -> Self { (self as u8 ^ 32) as char }
    
    fn as_lrud(mut self) -> usize { self = self.to_ascii_uppercase(); ['L', 'R', 'U', 'D'].into_iter().position(|v| v == self).unwrap() }
}



pub trait IntUtil: Copy {
    fn bit(self, n: usize) -> bool;
}

impl IntUtil for usize {
    fn bit(self, n: usize) -> bool { self>>n & 1 == 1 }
}



pub trait AsBounds: RangeBounds<usize> {
    /// `RangeBounds` を `st..ed` で表したときの `(st, ed)` を返す。
    /// 
    /// # Panics
    /// 
    /// `range` が `0..sup` に含まれないとき。
    fn as_bounds(&self, sup: usize) -> [usize; 2] {
        let l = match self.start_bound() {
            Included(&v) => v,
            Excluded(&v) => v+1,
            Unbounded => 0
        };
        
        let r = match self.end_bound() {
            Included(&v) => v+1,
            Excluded(&v) => v,
            Unbounded => sup
        };
        
        assert!(l <= r && r <= sup, "valid: 0..{sup}\ninputed: {l}..{r}");
        [l, r]
    }
}

impl<T: RangeBounds<usize>> AsBounds for T {}
