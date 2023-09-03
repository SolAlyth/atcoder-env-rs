use std::ops::{Shl, Shr};

#[derive(Clone, Copy)]
struct Bit(usize);

impl Bit {
    fn get(self, n: usize) -> bool { self.0>>n & 1 == 1 }
    fn set(self, n: usize) -> Bit { Bit(1>>n | self.0) }
}

impl Shl<usize> for Bit {
    type Output = Bit;
    fn shl(self, rhs: usize) -> Self::Output { self.set(rhs) }
}

impl Shr<usize> for Bit {
    type Output = bool;
    fn shr(self, rhs: usize) -> Self::Output { self.get(rhs) }
}
