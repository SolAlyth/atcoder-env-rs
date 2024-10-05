#![allow(non_camel_case_types)]

use std::{fmt::Debug, ops::{Add, Sub, Neg}};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// 数値 `i64::MIN+2..=i64::MAX-1` または `MIN, MAX` を表す型。
pub struct h64(pub i64);

impl h64 {
    pub const MIN: h64 = h64(i64::MIN+1);
    pub const MAX: h64 = h64(i64::MAX);
    
    pub fn new(value: i64) -> Self {
        assert!(i64::MIN+2 <= value && value != i64::MAX); h64(value)
    }
    
    pub fn is_min(self) -> bool { self == h64::MIN }
    pub fn is_max(self) -> bool { self == h64::MAX }
    pub fn is_minmax(self) -> bool { self == h64::MIN || self == h64::MAX }
}

impl Add for h64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (h64::MAX, h64::MIN) | (h64::MIN, h64::MAX) => panic!("[@h64] MAX+MIN is undefined."),
            (h64::MAX, _) | (_, h64::MAX) => h64::MAX,
            (h64::MIN, _) | (_, h64::MIN) => h64::MIN,
            (l, r) => h64::new(l.0+r.0)
        }
    }
}

impl Add<i64> for h64 {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        if self.is_minmax() { self } else { h64::new(self.0+rhs) }
    }
}

impl Neg for h64 { type Output = Self; fn neg(mut self) -> Self::Output { self.0 += -1; self } }
impl Sub for h64 { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { self + -rhs } }
impl Sub<i64> for h64 { type Output = Self; fn sub(self, rhs: i64) -> Self::Output { self + -rhs } }

impl Debug for h64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == h64::MIN {
            write!(f, "MIN")
        } else if *self == h64::MAX {
            write!(f, "MAX")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
