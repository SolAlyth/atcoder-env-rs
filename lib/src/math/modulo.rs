#![allow(non_upper_case_globals)]

pub const i998: i128 = 998244353;
pub const u998: u128 = 998244353;

pub trait Mod: Sized {
    fn normalize(&mut self) -> Self;
    fn mpow(self, p: u128) -> Self;
    fn inv(self) -> Self { self.mpow(u998-2) }
}

impl Mod for i128 {
    fn normalize(&mut self) -> Self {
        if !(0..i998).contains(self) { *self %= i998; if *self < 0 { *self += i998; } } *self
    }
    
    fn mpow(mut self, mut p: u128) -> Self {
        let mut out = 1;
        while p != 0 {
            if p&1 == 1 { out *= self; out.normalize(); }
            self *= self;
            self.normalize();
            p >>= 1;
        }
        out
    }
}

impl Mod for u128 {
    fn normalize(&mut self) -> Self {
        if u998 <= *self { *self %= u998; } *self
    }
    
    fn mpow(mut self, mut p: u128) -> Self {
        let mut out = 1;
        while p != 0 {
            if p&1 == 1 { out *= self; out.normalize(); }
            self *= self; self.normalize();
            p >>= 1;
        }
        out
    }
}
