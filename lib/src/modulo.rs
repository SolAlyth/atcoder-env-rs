#![allow(non_upper_case_globals)]

pub const i998: i128 = 998244353;
pub const u998: u128 = 998244353;

pub trait Mod {
    fn normalize(&self) -> Self;
    fn inv(&self) -> Self;
    fn mpow(self, p: Self) -> Self;
}

impl Mod for i128 {
    fn normalize(&self) -> Self {
        let a = self%i998;
        if a < 0 { a+i998 } else { a }
    }
    
    fn inv(&self) -> Self {
        self.mpow(i998-2)
    }
    
    fn mpow(mut self, mut p: Self) -> Self {
        let mut a = 1;
        while p != 0 {
            if p&1 == 1 { a = a*self % i998; }
            self = self*self % i998;
            p >>= 1;
        }
        a
    }
}
