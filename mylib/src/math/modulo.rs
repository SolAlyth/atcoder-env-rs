#![allow(non_upper_case_globals)]

use super::super::{imod, usmod};

pub trait Modulo: Copy {
    /// `[0, m)` に正規化する。
    fn simplify(self) -> Self;
    
    /// `pow(self, a) % m` を計算する。
    ///
    /// 繰り返し二乗法による実装。計算量 O(log p).
    fn mpow(self, a: usize) -> Self;
    
    /// 逆元を求める。`self mod m == 0` のとき `0` を返す。
    ///
    /// フェルマーの小定理による実装。計算量 O(log M). でもやり過ぎると TLE する。
    fn minv_fermat(self) -> Self { self.mpow(usmod-2) }
}

impl Modulo for i128 {
    fn simplify(mut self) -> Self {
        if !(0..imod).contains(&self) { self %= imod; if self < 0 { self += imod; } } self
    }
    
    fn mpow(mut self, mut a: usize) -> Self {
        let mut out = 1;
        while a != 0 {
            if a&1 == 1 { out = (out * self).simplify(); }
            self = self.pow(2).simplify();
            a >>= 1;
        }
        out
    }
}
