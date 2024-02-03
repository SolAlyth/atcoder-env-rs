use std::{fmt::Debug, ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}};
use num_integer::gcd;

/// 有理数を表す。
///
/// `self.0 / self.1` を表す。
///
/// # Guarantee
///
/// + 分母は 0 でない必要がある。
#[derive(Clone, Copy)]
pub struct Rational (i128, i128);

impl Rational {
    /// a/b と等価な既約分数を返す。
    ///
    /// # Panic
    ///
    /// + `b` が 0 である
    pub fn new(a: i128, b: i128) -> Self {
        assert_ne!(b, 0);
        Rational(a, b).simplify()
    }
    
    /// 等価な既約分数を返す。分母は正の整数であることが保証される。
    fn simplify(mut self) -> Self {
        if self.1 < 0 { self.0 *= -1; self.1 *= -1; }
        let gcd = gcd(self.0, self.1);
        self.0 /= gcd; self.1 /= gcd;
        self
    }
    
    pub fn set(&mut self, a: Option<i128>, b: Option<i128>) {
        if let Some(a) = a { self.0 = a; }
        if let Some(b) = b { assert_ne!(b, 0); self.1 = b; }
        *self = self.simplify();
    }
    
    /// 分子分母の値を返す。
    pub fn get(self) -> (i128, i128) { (self.0, self.1) }
    
    /// 逆数を返す。0 であるときは `None` を返す。
    pub fn inv(self) -> Option<Self> {
        if self.0 == 0 { None } else { Some(Rational(self.1, self.0)) }
    }
}

impl Neg for Rational { type Output = Self; fn neg(mut self) -> Self { self.0 *= -1; self } }
impl Add for Rational { type Output = Self; fn add(self, rhs: Self) -> Self { Rational(self.0 * rhs.1 + self.1 * rhs.0, self.1 * rhs.0).simplify() } }
impl AddAssign for Rational { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl Sub for Rational { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (-rhs) } }
impl SubAssign for Rational { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl Mul for Rational { type Output = Self; fn mul(self, rhs: Self) -> Self { Rational(self.0 * rhs.0, self.1 * rhs.1).simplify() } }
impl MulAssign for Rational { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
impl Div for Rational { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv().unwrap() } }
impl DivAssign for Rational { fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; } }

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
